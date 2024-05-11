use phper::{ini::Policy, modules::Module, php_get_module};

mod context;
mod errors;
mod hock;
mod module;
mod request;
mod span;
mod util;

const MINITRACE_LOG_FILE: &str = "minitrace.log_file";

/// This is the entry of php extension, the attribute macro `php_get_module`
/// will generate the `extern "C" fn`.
#[php_get_module]
pub fn get_module() -> Module {
    // New `Module` with extension info.
    let mut module = Module::new("minitrace", "0.1.0", "Orlion");

    module.add_ini(
        MINITRACE_LOG_FILE,
        "/tmp/minitrace.log".to_string(),
        Policy::System,
    );

    module.on_module_init(module::init);
    module.on_request_init(request::init);
    module.on_request_shutdown(request::shutdown);

    module
}
