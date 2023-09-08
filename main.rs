mod metrics;
use metrics::Metrics;

fn main() {
    Metrics::add_message("First, World!");
    Metrics::add_message("Second, World!");
    Metrics::add_message("Third, World!");

    Metrics::print_messages();
}