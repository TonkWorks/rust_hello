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
        let mut metrics = GLOBAL_METRICS.lock().unwrap();
        metrics._add_message(msg);
    }

    pub fn print_messages() {
        let metrics = GLOBAL_METRICS.lock().unwrap();
        metrics._print_messages();
    }
    
    // A method to add a message to the buffer
    fn _add_message(&mut self, msg: &str) {
        let mut messages = self.messages.lock().unwrap();
        messages.push(msg.to_string());
    }

    // A method to print all messages in the buffer
    fn _print_messages(&self) {
        let messages = self.messages.lock().unwrap();
        for msg in &*messages {
            println!("{}", msg);
        }
    }

    // A method to get the count of messages
    fn _message_count(&self) -> usize {
        let messages = self.messages.lock().unwrap();
        messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_message() {
        let mut metrics = Metrics::new();
        metrics._add_message("Test message");
        assert_eq!(metrics._message_count(), 1);
    }

    #[test]
    fn test_message_count() {
        let mut metrics = Metrics::new();
        assert_eq!(metrics._message_count(), 0);
        metrics._add_message("First message");
        metrics._add_message("Second message");
        assert_eq!(metrics._message_count(), 2);
    }
}
