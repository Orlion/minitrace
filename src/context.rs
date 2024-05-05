use uuid::Uuid;

static mut CONTEXT: Option<&crate::context::Context> = None;

pub struct Context {
    trace_id: String,
}

impl Context {
    pub fn new() -> Self {
        // Generate a unique trace_id
        Self { 
            trace_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }
}

pub fn create_context() {
    let ctx = Box::new(Context::new());
    unsafe {
        CONTEXT = Some(Box::leak(ctx));
    }
}

pub fn get_context() -> &'static Context {
    unsafe { CONTEXT.unwrap() }
}