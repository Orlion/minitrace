use phper::{
    sys, 
    values::ExecuteData,
    strings::ZStr,
};

pub fn init() {
    unsafe {
        sys::zend_observer_fcall_register(Some(observer_handler))
    }
}

pub unsafe extern "C" fn observer_handler(
    execute_data: *mut sys::zend_execute_data,
) -> sys::zend_observer_fcall_handlers {
    let Some(execute_data) = ExecuteData::try_from_mut_ptr(execute_data) else {
        return Default::default();
    };

    let (function_name, class_name) = match get_function_and_class_name(execute_data) {
        Ok(x) => x,
        Err(err) => {
            return Default::default();
        }
    };

    let Some(class_name) = class_name else {
        return Default::default();
    };

    let Some(function_name) = function_name else {
        return Default::default();
    };

    if class_name != "PDO" || function_name != "query"{
        return Default::default();
    }

    sys::zend_observer_fcall_handlers {
        begin: Some(observer_begin),
        end: Some(observer_end),
    }
}

fn get_function_and_class_name(
    execute_data: &mut ExecuteData,
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
        return Default::default();
    };

    let Some(function_name) = function_name else {
        return Default::default();
    };

    if class_name != "PDO" || function_name != "query"{
        return Default::default();
    }

    dbg!("PDO::query begin");
}

unsafe extern "C" fn observer_end(
    execute_data: *mut sys::zend_execute_data, retval: *mut sys::zval,
) {
    let Some(execute_data) = ExecuteData::try_from_mut_ptr(execute_data) else {
        return;
    };

    let Ok((function_name, class_name)) = get_function_and_class_name(execute_data) else {
        return;
    };

    let Some(class_name) = class_name else {
        return Default::default();
    };

    let Some(function_name) = function_name else {
        return Default::default();
    };

    if class_name != "PDO" || function_name != "query"{
        return Default::default();
    }

    dbg!("PDO::query end");
}