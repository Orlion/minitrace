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
