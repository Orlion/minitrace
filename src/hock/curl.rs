use super::HockSpan;
use crate::errors::Result;
use crate::span;
use phper::{
    objects::ZObj,
    values::{ExecuteData, ZVal},
};
use std::collections::HashMap;
use url::Url;

pub fn hock_before_curl_exec(function_name: &String, execute_data: &mut ExecuteData) -> HockSpan {
    if execute_data.num_args() < 1 {
        return HockSpan {
            kind: span::SPAN_KIND_CURL.to_string(),
            name: function_name.clone(),
            payload: HashMap::new(),
        };
    }

    let payload: Result<HashMap<String, String>> = {
        let ch = execute_data.get_parameter(0);
        let curl_info = phper::functions::call("curl_get_info", &mut [ch.clone()]).ok()?;
        let payload = HashMap::new();
        Ok(payload)
    };

    HockSpan {
        kind: span::SPAN_KIND_PDO.to_string(),
        name: "__construct".to_string(),
        payload: payload,
    }
}

fn get_curl_info(ch: ZVal) -> Option<HashMap<String, String>> {
    let curl_info = phper::functions::call("curl_get_info", &mut [ch.clone()]).ok()?;
    let curl_info = curl_info.as_z_arr()?;
    let url = curl_info.get("url")?.as_z_str()?.to_str().ok()?.to_string();
    let mut url: Url = url.parse().ok()?;
    url.set_query(None);
    url.set_fragment(None);
    let url = url.to_string();

    let http_code = curl_info.get("http_code")?.as_long()?;
}
