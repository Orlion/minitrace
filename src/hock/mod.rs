pub mod curl;
pub mod pdo;

use crate::context;
use crate::errors::Result;
use phper::{strings::ZStr, sys, values::ExecuteData, values::ZVal};
use std::collections::HashMap;

pub unsafe extern "C" fn observer_handler(
    execute_data: *mut sys::zend_execute_data,
) -> sys::zend_observer_fcall_handlers {
    let Some(execute_data) = ExecuteData::try_from_mut_ptr(execute_data) else {
        return Default::default();
    };

    let (function_name, class_name) = match get_function_and_class_name(execute_data) {
        Ok(x) => x,
        Err(_) => {
            return Default::default();
        }
    };

    if get_hock(&class_name, &function_name).is_none() {
        return Default::default();
    };

    sys::zend_observer_fcall_handlers {
        begin: Some(observer_begin),
        end: Some(observer_end),
    }
}

fn get_function_and_class_name(
    execute_data: &ExecuteData,
) -> anyhow::Result<(Option<String>, Option<String>)> {
    let function = execute_data.func();

    let function_name = function
        .get_function_name()
        .map(ZStr::to_str)
        .transpose()?
        .map(ToOwned::to_owned);
    let class_name = function
        .get_class()
        .map(|cls| cls.get_name().to_str().map(ToOwned::to_owned))
        .transpose()?;

    Ok((function_name, class_name))
}

unsafe extern "C" fn observer_begin(execute_data: *mut sys::zend_execute_data) {
    let Some(execute_data) = ExecuteData::try_from_mut_ptr(execute_data) else {
        return;
    };

    let Ok((function_name, class_name)) = get_function_and_class_name(execute_data) else {
        return;
    };

    if function_name.is_none() {
        return;
    }

    let hock = get_hock(&class_name, &function_name).unwrap();
    let hock_span = hock.0(&function_name.unwrap(), execute_data);
    context::get_context().start_span(&hock_span.kind, &hock_span.name, hock_span.payload);
}

unsafe extern "C" fn observer_end(
    execute_data: *mut sys::zend_execute_data,
    retval: *mut sys::zval,
) {
    let Some(execute_data) = ExecuteData::try_from_mut_ptr(execute_data) else {
        return;
    };

    let Ok((function_name, class_name)) = get_function_and_class_name(execute_data) else {
        return;
    };

    let mut null = ZVal::from(());
    let ret = match ZVal::try_from_mut_ptr(retval) {
        Some(ret) => ret,
        None => &mut null,
    };

    let ctx = context::get_context();

    let hock = get_hock(&class_name, &function_name).unwrap();
    if let Ok(payload) = hock.1(execute_data, ret) {
        ctx.extend_span_payload(payload);
    }

    ctx.end_span();
}

fn get_hock(
    class_name: &Option<String>,
    function_name: &Option<String>,
) -> Option<(Box<BeforeExecuteHook>, Box<AfterExecuteHook>)> {
    match (
        class_name.as_ref().map(|str: &String| str.as_str()),
        function_name.as_ref().map(|str: &String| str.as_str()),
    ) {
        (Some("PDO"), Some("__construct")) => Some((
            Box::new(pdo::hock_before_pdo_construct),
            Box::new(pdo::hock_after_pdo),
        )),
        (Some("PDO"), Some(f))
            if [
                "exec",
                "query",
                "prepare",
                "commit",
                "beginTransaction",
                "rollBack",
            ]
            .contains(&f) =>
        {
            Some((
                Box::new(pdo::hock_before_pdo_method),
                Box::new(pdo::hock_after_pdo),
            ))
        }
        (Some("PDOStatement"), Some(f))
            if ["execute", "fetch", "fetchAll", "fetchColumn", "fetchObject"].contains(&f) =>
        {
            Some((
                Box::new(pdo::hock_before_pdo_statement_method),
                Box::new(pdo::hock_after_pdo),
            ))
        }
        (_, Some("curl_exec")) => Some((
            Box::new(curl::hock_before_curl),
            Box::new(curl::hock_after_curl),
        )),
        _ => None,
    }
}

pub struct HockSpan {
    kind: String,
    name: String,
    payload: HashMap<String, String>,
}

pub type BeforeExecuteHook = dyn Fn(&String, &mut ExecuteData) -> HockSpan;
pub type AfterExecuteHook = dyn Fn(&mut ExecuteData, &mut ZVal) -> Result<HashMap<String, String>>;
