use metrics_derive::Metrics;
use prometheus_client::metrics::{
    counter::Counter,
    gauge::Gauge,
    histogram::Histogram,
};

#[allow(dead_code)]
#[derive(Metrics)]
#[metrics(namespace = "mycustommetric")]
pub struct MyCustomMetric {
    #[metrics(name = "errors", help = "Number of errors encountered")]
    errors: Counter,

    #[metrics(name = "requests", help = "Number of requests handled")]
    requests: Counter,

    #[metrics(
        name = "latency",
        help = "Latency of requests",
        buckets = "0.5, 0.9, 0.99"
    )]
    latency: Histogram,
}

#[allow(dead_code)]
#[derive(Metrics)]
pub struct MyOtherMetric {
    #[metrics(name = "errors", help = "Number of errors")]
    errors: Gauge,
}

#[allow(dead_code)]
#[derive(Metrics)]
#[metrics(prefix = "myprefix")]
pub struct MyOtherMetricWithPrefix {
    #[metrics(name = "errors", help = "Number of errors")]
    errors: Gauge,
}

fn main() {}
