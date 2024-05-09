use chrono::{offset::Local, DateTime};
use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use std::collections::HashMap;

pub const SPAN_KIND_URL: &str = "URL";
pub const SPAN_KIND_PDO: &str = "PDO";
pub const SPAN_KIND_CURL: &str = "CURL";

#[derive(Debug)]
pub struct Span {
    trace_id: String,
    kind: String,
    name: String,
    payload: HashMap<String, String>,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    duration_in_micro: i64,
}

impl Span {
    pub fn new(
        trace_id: String,
        kind: &str,
        name: &str,
        payload: HashMap<String, String>,
    ) -> Box<Self> {
        let now = Local::now();
        Box::new(Self {
            trace_id,
            kind: kind.to_string(),
            name: name.to_string(),
            payload,
            start_time: now,
            end_time: now,
            duration_in_micro: 0,
        })
    }

    pub fn extend_payload(&mut self, extend: HashMap<String, String>) {
        self.payload.extend(extend);
    }

    pub fn end(&mut self) {
        self.end_time = Local::now();
        self.duration_in_micro =
            self.end_time.timestamp_micros() - self.start_time.timestamp_micros();
    }

    pub fn set_trace_id(&mut self, trace_id: &str) {
        self.trace_id = trace_id.to_string();
    }
}

impl Serialize for Span {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Span", 6)?;
        state.serialize_field("trace_id", &self.trace_id)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("payload", &self.payload)?;
        state.serialize_field(
            "start_time",
            &self.start_time.format("%H:%M:%S%.3f").to_string(),
        )?;
        state.serialize_field(
            "end_time",
            &self.end_time.format("%H:%M:%S%.3f").to_string(),
        )?;
        state.serialize_field("duration_in_micro", &self.duration_in_micro)?;
        state.end()
    }
}
