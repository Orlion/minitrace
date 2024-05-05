use std::collections::HashMap;

use phper::{arrays::ZArr, eg};
use crate::util::z_val_to_string;
use crate::context;
use crate::span;

pub fn init() {
    context::create_context();

    let get = get_page_request_get();
    let post = get_page_request_post();
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

    context::get_context().start_span(span::SPAN_KIND_URL, &uri, payload);
}

pub fn shutdown() {
    let ctx = context::get_context();
    ctx.end_span();
    ctx.flush();
}

fn get_page_request_server<'a>() -> Option<&'a ZArr> {
    unsafe {
        let symbol_table = ZArr::from_mut_ptr(&mut eg!(symbol_table));
        symbol_table.get("_SERVER").and_then(|carrier| carrier.as_z_arr())
    }
}

fn get_page_request_get<'a>() -> Option<&'a ZArr> {
    unsafe {
        let symbol_table = ZArr::from_mut_ptr(&mut eg!(symbol_table));
        symbol_table.get("_GET").and_then(|carrier| carrier.as_z_arr())
    }
}

fn get_page_request_post<'a>() -> Option<&'a ZArr> {
    unsafe {
        let symbol_table = ZArr::from_mut_ptr(&mut eg!(symbol_table));
        symbol_table.get("_POST").and_then(|carrier| carrier.as_z_arr())
    }
}

fn get_page_request_uri(server: &ZArr) -> String {
    server
        .get("REQUEST_URI")
        .and_then(z_val_to_string)
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

fn get_page_request_method(server: &ZArr) -> String {
    server
        .get("REQUEST_METHOD")
        .and_then(z_val_to_string)
        .unwrap_or_else(|| "UNKNOWN".to_string())
}
