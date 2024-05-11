use std::collections::HashMap;

use crate::context;
use crate::span;
use crate::util::{z_str_to_string, z_val_to_json_str};
use phper::values::ZVal;
use phper::{arrays::ZArr, eg, sg};

pub fn init() {
    context::create_context();

    let get = get_page_request_get().unwrap();
    let post = get_page_request_post().unwrap();
    let server = get_page_request_server();
    let mut uri = String::from("UNKNOWN");
    let mut method = String::from("UNKNOWN");
    if let Some(server) = server {
        uri = get_page_request_uri(server);
        method = get_page_request_method(server);
    }

    let mut payload = HashMap::new();
    payload.insert("$_GET".to_string(), format!("{:?}", get));
    payload.insert("$_POST".to_string(), format!("{:?}", post));
    payload.insert("method".to_string(), method);

    let ctx = context::get_context();

    let _ = phper::functions::call(
        "header",
        &mut [ZVal::from(
            "X-MiniTrace-Id: ".to_string() + ctx.get_trace_id(),
        )],
    );

    context::get_context().start_root_span(span::SPAN_KIND_URL, &uri, payload);
}

pub fn shutdown() {
    let status_code = unsafe { sg!(sapi_headers).http_response_code };

    let ctx = context::get_context();
    ctx.extend_root_span_payload({
        let mut payload = HashMap::new();
        payload.insert("status_code".to_string(), status_code.to_string());
        payload
    });
    ctx.end_root_span();
    let _ = ctx.flush();
}

fn get_page_request_server<'a>() -> Option<&'a ZArr> {
    unsafe {
        let symbol_table = ZArr::from_mut_ptr(&mut eg!(symbol_table));
        symbol_table
            .get("_SERVER")
            .and_then(|carrier| carrier.as_z_arr())
    }
}

fn get_page_request_get() -> Option<String> {
    unsafe {
        let symbol_table = ZArr::from_mut_ptr(&mut eg!(symbol_table));
        symbol_table
            .get("_GET")
            .and_then(|carrier| z_val_to_json_str(carrier))
    }
}

fn get_page_request_post() -> Option<String> {
    unsafe {
        let symbol_table = ZArr::from_mut_ptr(&mut eg!(symbol_table));
        symbol_table
            .get("_POST")
            .and_then(|carrier| z_val_to_json_str(carrier))
    }
}

fn get_page_request_uri(server: &ZArr) -> String {
    server
        .get("REQUEST_URI")
        .and_then(z_str_to_string)
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

fn get_page_request_method(server: &ZArr) -> String {
    server
        .get("REQUEST_METHOD")
        .and_then(z_str_to_string)
        .unwrap_or_else(|| "UNKNOWN".to_string())
}
