use std::collections::HashMap;

use phper::{functions::Argument, ini::Policy, modules::Module, php_get_module, values::ZVal};

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

    module
        .add_function("minitrace_error_handler", minitrace_error_handler)
        .arguments(vec![
            Argument::by_val("errno"),
            Argument::by_val("errstr"),
            Argument::by_val("errfile"),
            Argument::by_val("errline"),
        ]);

    module
        .add_function("minitrace_exception_handler", minitrace_exception_handler)
        .arguments(vec![Argument::by_val("ex")]);

    module.on_module_init(module::init);
    module.on_request_init(request::init);
    module.on_request_shutdown(request::shutdown);

    module
}

fn minitrace_error_handler(arguments: &mut [ZVal]) -> phper::Result<()> {
    let errno = arguments[0].expect_long()?;
    let errstr = arguments[1].expect_z_str()?.to_str()?;
    let errfile = arguments[2].expect_z_str()?.to_str()?;
    let errline = arguments[3].expect_long()?;

    let ctx = context::get_context();
    ctx.start_span(
        crate::span::SPAN_KIND_ERROR,
        format!(
            "{}: {} in {} on line {}",
            util::errno_2_str(errno),
            errstr,
            errfile,
            errline
        )
        .as_str(),
        HashMap::new(),
    );
    ctx.end_span();
    Ok(())
}

fn minitrace_exception_handler(arguments: &mut [ZVal]) -> phper::Result<()> {
    let ex = arguments[0].expect_mut_z_obj()?;
    let binding = ex.call("getMessage", &mut [])?;
    let message = binding
        .as_z_str()
        .ok_or(phper::errors::NotImplementThrowableError)?
        .to_str()?;
    let binding = ex.call("getFile", &mut [])?;
    let file = binding
        .as_z_str()
        .ok_or(phper::errors::NotImplementThrowableError)?
        .to_str()?;
    let line = ex
        .call("getLine", &mut [])?
        .as_long()
        .ok_or(phper::errors::NotImplementThrowableError)?;
    let binding = ex.call("getTraceAsString", &mut [])?;
    let trace = binding
        .as_z_str()
        .ok_or(phper::errors::NotImplementThrowableError)?
        .to_str()?;

    let class_name = ex.get_class().get_name().to_str()?;

    let mut payload = HashMap::new();
    payload.insert("trace".to_string(), trace.to_string());

    let ctx = context::get_context();
    ctx.start_span(
        crate::span::SPAN_KIND_EXCEPTION,
        format!("{}: {} in {} on line {}", class_name, message, file, line).as_str(),
        payload,
    );
    ctx.end_span();
    Ok(())
}
