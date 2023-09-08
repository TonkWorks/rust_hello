use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_METRICS: Mutex<Metrics> = Mutex::new(Metrics::new());
}

pub struct Metrics {
    messages: Mutex<Vec<String>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            messages: Mutex::new(Vec::new()),
        }
    }

    pub fn add_message(msg: &str) {
        let metrics = GLOBAL_METRICS.lock().unwrap();
        let mut messages = metrics.messages.lock().unwrap();
        messages.push(msg.to_string());
    }

    pub fn print_messages() {
        let metrics = GLOBAL_METRICS.lock().unwrap();
        let mut messages = metrics.messages.lock().unwrap();
        for msg in &*messages {
            println!("{}", msg);
        }
        messages.clear();
    }
}

#[test]
fn test_print_messages() {
    Metrics::add_message("Test message 1");
    Metrics::add_message("Test message 2");

    let before_print = {
        let metrics = GLOBAL_METRICS.lock().unwrap();
        let messages = metrics.messages.lock().unwrap();
        messages.len()
    };

    Metrics::print_messages();

    let after_print = {
        let metrics = GLOBAL_METRICS.lock().unwrap();
        let messages = metrics.messages.lock().unwrap();
        messages.len()
    };

    assert_eq!(before_print, 2);
    assert_eq!(after_print, 0); 
}