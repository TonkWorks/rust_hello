mod metrics;

fn main() {
    // Add some messages to the buffer
    // METRICS.add_message("First message");
    // METRICS.add_message("Second message");
    // METRICS.add_message("Third message");
    metrics::Metrics::add_message("First, World!");
    metrics::Metrics::add_message("Second, World!");
    metrics::Metrics::add_message("Third, World!");

    metrics::Metrics::print_messages();
}