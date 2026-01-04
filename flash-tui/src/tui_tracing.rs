use std::sync::{Arc, Mutex};
use tracing::{Level, Subscriber};
use tracing_subscriber::{Layer, layer::Context, registry::LookupSpan};

// Log entry structure
#[derive(Clone)]
pub struct LogEntry {
    pub level: Level,
    pub message: String,
}

// Log buffer for storing tracing output
pub struct LogBuffer {
    entries: Arc<Mutex<Vec<LogEntry>>>,
}

impl LogBuffer {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_entry(&self, level: Level, message: String) {
        let mut entries = self.entries.lock().unwrap();
        entries.push(LogEntry { level, message });
    }

    pub fn get_entries(&self) -> Vec<LogEntry> {
        self.entries.lock().unwrap().clone()
    }
}

// Custom tracing layer that writes to LogBuffer
pub struct TuiTracingLayer {
    log_buffer: Arc<LogBuffer>,
}

impl TuiTracingLayer {
    pub fn new(log_buffer: Arc<LogBuffer>) -> Self {
        Self { log_buffer }
    }
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for TuiTracingLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let level = *metadata.level();

        // Format the event message
        let mut visitor = MessageVisitor::new();
        event.record(&mut visitor);

        let final_message = if visitor.message.is_empty() {
            format!("{}", metadata.name())
        } else {
            visitor.message
        };

        // Append additional fields if present
        let final_message = if !visitor.fields.is_empty() {
            let fields_str: Vec<String> = visitor
                .fields
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            format!("{} {}", final_message, fields_str.join(" "))
        } else {
            final_message
        };

        self.log_buffer.add_entry(level, final_message);
    }
}

// Visitor to extract message from event
struct MessageVisitor {
    message: String,
    fields: Vec<(String, String)>,
}

impl MessageVisitor {
    fn new() -> Self {
        Self {
            message: String::new(),
            fields: Vec::new(),
        }
    }
}

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        } else {
            self.fields
                .push((field.name().to_string(), format!("{:?}", value)));
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        } else {
            self.fields
                .push((field.name().to_string(), value.to_string()));
        }
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if field.name() != "message" {
            self.fields
                .push((field.name().to_string(), value.to_string()));
        }
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        if field.name() != "message" {
            self.fields
                .push((field.name().to_string(), value.to_string()));
        }
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        if field.name() != "message" {
            self.fields
                .push((field.name().to_string(), value.to_string()));
        }
    }
}

