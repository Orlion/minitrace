use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const SPAN_KIND_URL: &str = "URL";

#[derive(Serialize, Deserialize)]
pub struct Span {
    trace_id: String,
    kind: String,
    name: String,
    payload: HashMap<String, String>,
    start_time: u64,
    end_time: u64,
}

impl Span {
    pub fn new(kind: &str, name: &str, payload: HashMap<String, String>) -> Self {
        Self {
            trace_id: String::new(),
            kind: kind.to_string(),
            name: name.to_string(),
            payload,
        }
    }

    pub fn append_payload(&mut self, key: &str, value: &str) {
        self.payload.insert(key.to_string(), value.to_string());
    }

    pub fn end(&self) {
        
    }

    pub fn set_trace_id(&mut self, trace_id: &str) {
        self.trace_id = trace_id.to_string();
    }
}