use super::HockSpan;
use crate::errors::Result;
use crate::span;
use phper::values::{ExecuteData, ZVal};
use std::collections::HashMap;
use url::Url;

pub fn hock_before_curl(function_name: &String, execute_data: &mut ExecuteData) -> HockSpan {
    let mut payload = HashMap::new();
    let mut name = String::from("UNKNOWN");
    dbg!(execute_data.num_args());
    if execute_data.num_args() > 0 {
        let ch = execute_data.get_parameter(0);
        if let Some((url, query)) = get_url_from_curl_handle(ch) {
            name = url;
            payload.insert("query".to_string(), query);
        }
    }

    HockSpan {
        kind: span::SPAN_KIND_CURL.to_string(),
        name,
        payload,
    }
}

pub fn hock_after_curl(
    execute_data: &mut ExecuteData,
    return_value: &mut ZVal,
) -> Result<HashMap<String, String>> {
    let mut payload = HashMap::new();
    if execute_data.num_args() > 0 {
        let ch = execute_data.get_parameter(0);
        if let Some((http_code, curl_error)) = get_curl_info_from_curl_handle(ch) {
            payload.insert("http_code".to_string(), http_code.to_string());
            payload.insert("curl_error".to_string(), curl_error);
        }
    }

    Ok(payload)
}

fn get_url_from_curl_handle(ch: &ZVal) -> Option<(String, String)> {
    let curl_info = phper::functions::call("curl_getinfo", &mut [ch.clone()]).ok()?;
    let curl_info = curl_info.as_z_arr()?;
    let url = curl_info.get("url")?.as_z_str()?.to_str().ok()?.to_string();
    let mut url: Url = url.parse().ok()?;
    let mut query_str = String::from("");
    if let Some(query) = url.query() {
        query_str = query.to_string()
    }

    url.set_query(None);
    url.set_fragment(None);
    let url = url.to_string();
    Some((url, query_str))
}

fn get_curl_info_from_curl_handle(ch: &ZVal) -> Option<(i64, String)> {
    let mut curl_error = String::from("");
    let curl_info = phper::functions::call("curl_get_info", &mut [ch.clone()]).ok()?;
    let http_code = curl_info.as_z_arr()?.get("http_code")?.as_long()?;
    if http_code == 0 {
        curl_error = phper::functions::call("curl_error", &mut [ch.clone()])
            .ok()?
            .as_z_str()?
            .to_str()
            .ok()?
            .to_string();
    }

    Some((http_code, curl_error))
}
