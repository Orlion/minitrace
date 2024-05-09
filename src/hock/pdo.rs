use super::HockSpan;
use crate::errors::Result;
use crate::span;
use phper::{
    objects::ZObj,
    values::{ExecuteData, ZVal},
};
use std::collections::HashMap;

pub fn hock_before_pdo_construct(
    _function_name: &String,
    execute_data: &mut ExecuteData,
) -> HockSpan {
    let dsn = execute_data.get_parameter(0);
    let dsn = dsn.as_z_str();
    let dsn = match dsn {
        Some(dsn) => match dsn.to_str() {
            Ok(dsn) => dsn.to_string(),
            Err(_) => "can't get dsn".to_string(),
        },
        None => "dsn isn't str".to_string(),
    };
    HockSpan {
        kind: span::SPAN_KIND_PDO.to_string(),
        name: "__construct".to_string(),
        payload: {
            let mut payload = HashMap::new();
            payload.insert("dsn".to_string(), dsn);
            payload
        },
    }
}

pub fn hock_before_pdo_method(function_name: &String, execute_data: &mut ExecuteData) -> HockSpan {
    let mut statement_str = String::new();
    if execute_data.num_args() >= 1 {
        if let Some(statement) = execute_data.get_parameter(0).as_z_str() {
            if let Ok(statement) = statement.to_str() {
                statement_str = statement.to_string();
            }
        }
    }
    HockSpan {
        kind: span::SPAN_KIND_PDO.to_string(),
        name: function_name.clone(),
        payload: {
            let mut payload = HashMap::new();
            payload.insert("statement".to_string(), statement_str);
            payload
        },
    }
}

pub fn hock_before_pdo_statement_method(
    function_name: &String,
    execute_data: &mut ExecuteData,
) -> HockSpan {
    let mut query_string = String::new();

    let this = execute_data.get_this_mut();
    if let Some(this) = this {
        if let Some(query) = this.get_property("queryString").as_z_str() {
            if let Ok(query) = query.to_str() {
                query_string = query.to_string();
            }
        }
    }

    HockSpan {
        kind: span::SPAN_KIND_PDO.to_string(),
        name: function_name.clone(),
        payload: {
            let mut payload = HashMap::new();
            payload.insert("query_string".to_string(), query_string);
            payload
        },
    }
}

pub fn hock_after_pdo(
    execute_data: &mut ExecuteData,
    return_value: &mut ZVal,
) -> Result<HashMap<String, String>> {
    let mut payload = HashMap::new();
    if let Some(b) = return_value.as_bool() {
        // pdo method return false, so we need to get error info
        if !b {
            if let Some(error_info) = get_pdo_error_info(
                execute_data
                    .get_this_mut()
                    .ok_or("this is null".to_string())?,
            ) {
                payload.insert("state".to_string(), error_info.0.to_string());
                payload.insert("code".to_string(), error_info.1.to_string());
                payload.insert("error".to_string(), error_info.2.to_string());
            }
        }
    } else if let Some(obj) = return_value.as_mut_z_obj() {
        let class_name = obj.get_class().get_name().to_str()?;
        payload.insert("class_name".to_string(), class_name.to_string());
    } else if let Some(i) = return_value.as_long() {
        // pdo method return int
        payload.insert("return_value".to_string(), i.to_string());
    } else if let Some(arr) = return_value.as_mut_z_arr() {
        // fetch array length
        payload.insert("fetch_array_length".to_string(), arr.len().to_string());
    }

    Ok(payload)
}

fn get_pdo_error_info(this: &mut ZObj) -> Option<(&str, i64, &str)> {
    let info = this.call("errorInfo", []).ok()?;
    let info = info.as_z_arr()?;

    let state = info.get(0)?.expect_z_str().ok()?.to_str().ok()?;
    let code = {
        let code = info.get(1)?;
        // PDOStatement::fetch
        // In all cases, false is returned on failure or if there are no more rows.
        if code.get_type_info().is_null() {
            return None;
        }

        code.expect_long().ok()?
    };
    let error = info.get(2)?.expect_z_str().ok()?.to_str().ok()?;
    Some((state, code, error))
}

#[derive(Debug)]
enum Error {
    PHPer(phper::Error),
    Anyhow(anyhow::Error),
}
