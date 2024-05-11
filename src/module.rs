use std::ffi::CStr;

use phper::{ini::ini_get, sys};

use crate::{hock, MINITRACE_LOG_FILE};

pub static mut LOG_FILE: &str = "";

pub fn init() {
    let log_file = ini_get::<Option<&CStr>>(MINITRACE_LOG_FILE)
        .and_then(|s| s.to_str().ok())
        .unwrap_or_default();

    let log_file = log_file.trim();

    unsafe { LOG_FILE = log_file };

    unsafe { sys::zend_observer_fcall_register(Some(hock::observer_handler)) }
}
