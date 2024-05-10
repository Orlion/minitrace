use crate::span::Span;
use std::io::Write;
use std::{collections::HashMap, fs::OpenOptions};
use uuid::Uuid;

static mut CONTEXT: Option<&mut Context> = None;

pub struct Context {
    trace_id: String,
    spans: Vec<Box<Span>>,
    root: Option<Box<Span>>,
    top: Option<Box<Span>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            trace_id: Uuid::new_v4().to_string(),
            spans: Vec::new(),
            root: None,
            top: None,
        }
    }

    pub fn get_trace_id(&self) -> &str {
        &self.trace_id
    }

    pub fn start_root_span(&mut self, kind: &str, name: &str, payload: HashMap<String, String>) {
        if self.root.is_some() {
            panic!("Cannot start a new span while another is active");
        }
        self.root = Some(Span::new(self.trace_id.clone(), kind, name, payload));
    }

    pub fn end_root_span(&mut self) {
        let mut span = self.root.take().unwrap();
        span.end();
        self.spans.push(span);
        self.root = None;
    }

    pub fn start_span(&mut self, kind: &str, name: &str, payload: HashMap<String, String>) {
        if self.top.is_some() {
            panic!("Cannot start a new span while another is active");
        }
        self.top = Some(Span::new(self.trace_id.clone(), kind, name, payload));
    }

    pub fn end_span(&mut self) {
        let mut span = self.top.take().unwrap();
        span.end();
        self.spans.push(span);
    }

    pub fn flush(&self) -> crate::errors::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("/tmp/minitrace.log")?;

        for span in &self.spans {
            if let Ok(json) = serde_json::to_string(span) {
                let _ = writeln!(file, "{}", json);
            }
        }

        Ok(())
    }

    pub fn extend_span_payload(&mut self, extend: HashMap<String, String>) {
        self.top.as_mut().unwrap().extend_payload(extend);
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
    unsafe { CONTEXT.as_mut().unwrap() }
}
