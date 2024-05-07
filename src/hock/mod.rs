pub mod pdo;

use crate::context;
use phper::{strings::ZStr, sys, values::ExecuteData};
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

    let Some(class_name) = class_name else {
        return Default::default();
    };

    let Some(function_name) = function_name else {
        return Default::default();
    };

    let Some(_) = get_hock(&class_name, &function_name) else {
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

    let Some(class_name) = class_name else {
        return;
    };

    let Some(function_name) = function_name else {
        return;
    };

    let hock = get_hock(class_name.as_str(), function_name.as_str()).unwrap();
    let hock_span = hock.0(&function_name, execute_data);
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

    let Some(class_name) = class_name else {
        return;
    };

    let Some(function_name) = function_name else {
        return;
    };

    let hock = get_hock(class_name.as_str(), function_name.as_str()).unwrap();
    hock.1(execute_data);

    context::get_context().end_span();
}

fn get_hock(
    class_name: &str,
    function_name: &str,
) -> Option<(Box<BeforeExecuteHook>, Box<AfterExecuteHook>)> {
    match (class_name, function_name) {
        ("PDO", "__construct") => Some((
            Box::new(pdo::hock_before_pdo_construct),
            Box::new(hock_after_common),
        )),
        ("PDO", f)
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
                Box::new(hock_after_common),
            ))
        }
        _ => None,
    }
}

pub struct HockSpan {
    kind: String,
    name: String,
    payload: HashMap<String, String>,
}

pub type BeforeExecuteHook = dyn Fn(&String, &ExecuteData) -> HockSpan;
pub type AfterExecuteHook = dyn Fn(&ExecuteData);

fn hock_after_common(execute_data: &ExecuteData) {}
