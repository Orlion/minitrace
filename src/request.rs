use std::collections::HashMap;

use crate::context;
use crate::span;
use crate::util::{z_str_to_string, z_val_to_json_str};
use phper::{arrays::ZArr, eg, pg, sg, sys, values::ZVal};

pub fn init() {
    let _ = phper::functions::call(
        "set_error_handler",
        &mut [ZVal::from("minitrace_error_handler")],
    );

    let _ = phper::functions::call(
        "set_exception_handler",
        &mut [ZVal::from("minitrace_exception_handler")],
    );

    jit_initialization();

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
    payload.insert("$_GET".to_string(), get);
    payload.insert("$_POST".to_string(), post);
    payload.insert("method".to_string(), method);

    context::create_context();
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

#[allow(clippy::useless_conversion)]
fn jit_initialization() {
    unsafe {
        let jit_initialization: u8 = pg!(auto_globals_jit).into();
        if jit_initialization != 0 {
            let mut server = "_SERVER".to_string();
            sys::zend_is_auto_global_str(server.as_mut_ptr().cast(), server.len());
        }
    }
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
        .and_then(|uri: String| {
            let pos = uri.find('?').unwrap_or(uri.len());
            Some(uri.split_at(pos).0.to_string())
        })
        .unwrap_or("UNKNOWN".to_string())
}

fn get_page_request_method(server: &ZArr) -> String {
    server
        .get("REQUEST_METHOD")
        .and_then(z_str_to_string)
        .unwrap_or("UNKNOWN".to_string())
}
