use metrics_derive::Metrics;
use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{
        family::Family,
        gauge::Gauge,
    },
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct InfoLabels {
    version: String,
}

#[allow(dead_code)]
#[derive(Metrics)]
#[metrics(namespace = "mycustommetric")]
pub struct MyCustomMetric {
    #[metrics(name = "info", help = "Information about the exporter")]
    info: Family<InfoLabels, Gauge>,
}
