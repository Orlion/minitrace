use super::HockSpan;
use crate::span;
use phper::values::ExecuteData;
use std::collections::HashMap;

pub fn hock_before_pdo_construct(function_name: &String, execute_data: &ExecuteData) -> HockSpan {
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

pub fn hock_before_pdo_method(function_name: &String, execute_data: &ExecuteData) -> HockSpan {
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
