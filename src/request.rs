use phper::{arrays::ZArr, values::ZVal, eg};
use anyhow::Context;
use crate::util::z_val_to_string;
use crate::context;

pub fn init() {
    let get = get_page_request_get();
    let post = get_page_request_post();
    let server = get_page_request_server();
    let mut uri = String::from("UNKNOWN");
    let mut method = String::from("UNKNOWN");
    if let Some(server) = server {
        uri = get_page_request_uri(server);
        method = get_page_request_method(server);
    }

    context::create_context();
    let trace_id = context::get_context().trace_id();
    dbg!("request.init", trace_id, uri, method, get, post);
}

pub fn shutdown() {
    let trace_id = context::get_context().trace_id();
    dbg!("request.shutdown", trace_id);
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
