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
    #[metrics(
        name = "info",
        help = "Information about the exporter"
        init = InfoLabels { version: "1.0.0".to_string() }
        set = 1
    )]
    info: Family<InfoLabels, Gauge>,
}
