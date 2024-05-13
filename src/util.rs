use phper::{functions::call, values::ZVal};

pub fn z_str_to_string(zv: &ZVal) -> Option<String> {
    zv.as_z_str()
        .and_then(|zs| zs.to_str().ok())
        .map(|s| s.to_string())
}

pub fn z_val_to_json_str(zv: &ZVal) -> Option<String> {
    if let Ok(json) = call("json_encode", &mut [zv.clone()]) {
        z_str_to_string(&json)
    } else {
        None
    }
}

pub fn errno_2_str(errno: i64) -> String {
    match errno {
        1 => "E_ERROR",
        2 => "E_WARNING",
        4 => "E_PARSE",
        8 => "E_NOTICE",
        16 => "E_CORE_ERROR",
        32 => "E_CORE_WARNING",
        64 => "E_COMPILE_ERROR",
        128 => "E_COMPILE_WARNING",
        256 => "E_USER_ERROR",
        512 => "E_USER_WARNING",
        1024 => "E_USER_NOTICE",
        2048 => "E_STRICT",
        4096 => "E_RECOVERABLE_ERROR",
        8192 => "E_DEPRECATED",
        16384 => "E_USER_DEPRECATED",
        32767 => "E_ALL",
        _ => "UNKNOWN",
    }
    .to_string()
}
