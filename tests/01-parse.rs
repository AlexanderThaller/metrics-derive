use metrics_derive::Metrics;
use prometheus_client::metrics::{
    counter::Counter,
    gauge::Gauge,
};

#[allow(dead_code)]
#[derive(Metrics)]
#[metrics(namespace = "mycustommetric")]
pub struct MyCustomMetric {
    #[metrics(name = "errors", help = "Number of errors encountered")]
    errors: Counter,

    #[metrics(name = "requests", help = "Number of requests handled")]
    requests: Counter,
}

#[allow(dead_code)]
#[derive(Metrics)]
pub struct MyOtherMetric {
    #[metrics(name = "errors", help = "Number of errors")]
    errors: Gauge,
}

fn main() {}
