use std::time::Duration;

use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::resource::{OsResourceDetector, ProcessResourceDetector};
use opentelemetry_sdk::trace::{config, BatchConfig, Sampler};
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use opentelemetry_semantic_conventions::SCHEMA_URL;
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use url::Url;

/// Construct the tracing with oltp connector
pub fn init_trace_oltp(endpoint: &Url) -> Result<(), Box<dyn std::error::Error>> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint.as_str()),
        )
        .with_trace_config(
            config()
                .with_resource(Resource::from_detectors(
                    Duration::from_secs(2),
                    vec![
                        Box::new(OsResourceDetector),
                        Box::new(ProcessResourceDetector),
                    ],
                ))
                .with_resource(Resource::from_schema_url(
                    [KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME"))],
                    SCHEMA_URL,
                ))
                .with_sampler(Sampler::AlwaysOn),
        )
        .with_batch_config(BatchConfig::default())
        .install_batch(runtime::Tokio)?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::INFO))
        .with(OpenTelemetryLayer::new(tracer).with_filter(LevelFilter::DEBUG))
        .init();

    Ok(())
}
