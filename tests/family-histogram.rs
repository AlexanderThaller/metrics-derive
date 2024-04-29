use metrics_derive::Metrics;
use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{
        family::Family,
        histogram::Histogram,
    },
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ProcessingTimeLabels {
    method: String,
}

#[allow(dead_code)]
#[derive(Metrics)]
#[metrics(namespace = "mycustommetric")]
pub struct MyCustomMetric {
    #[metrics(name = "processing_time", help = "Processing time of requests")]
    processing_time: Family<ProcessingTimeLabels, Histogram>,
}

fn main() {}
