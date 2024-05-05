use uuid::Uuid;
use crate::span::Span;
use std::collections::HashMap;

static mut CONTEXT: Option<& mut Context> = None;

pub struct Context {
    trace_id: String,
    messages: Vec<Box<Span>>,
    top: Option<Box<Span>>,
}

impl Context {
    pub fn new() -> Self {
        Self { 
            trace_id: Uuid::new_v4().to_string(),
            messages: Vec::new(),
            top: None,
        }
    }

    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    pub fn start_span(&mut self, kind: &str, name: &str, payload: HashMap<String, String>) {
        if self.top.is_some() {
            panic!("Cannot start a new span while another is active");
        }
        self.top = Some(Box::new(Span::new(kind, name, payload)));
    }

    pub fn end_span(&mut self) {
        let span = self.top.take().unwrap();
        span.end();
        self.messages.push(span);
        self.top = None;
    }

    pub fn flush(&self) {
        for span in &self.messages {
            let json = serde_json::to_string(span).unwrap();
            dbg!(json);
        }
    }
}

// 创建一个Context实例，赋值给CONTEXT， 并返回一个指向CONTEXT的引用
pub fn create_context() {
    let ctx = Box::new(Context::new());
    unsafe {
        CONTEXT = Some(Box::leak(ctx));
    }
}

pub fn get_context() -> &'static mut Context {
    unsafe {
        CONTEXT.as_mut().unwrap()
    }
}