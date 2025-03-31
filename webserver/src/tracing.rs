//! Utilities for configuring tracing

use opentelemetry::trace::TraceError;
use opentelemetry::trace::TracerProvider;
use opentelemetry::Key;
use opentelemetry::KeyValue;
use opentelemetry::Value;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace;
use opentelemetry_sdk::Resource;
use tracing::Subscriber;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

use crate::config::OTEL_EXPORTER_OTLP_ENDPOINT;

/// Tracing layer exporting OpenTelemetry traces
///
/// The layer is not configurable yet and only suited for development.
/// It should simply work out of the box with a local jaeger instance.
pub fn opentelemetry_layer<S: Subscriber + for<'span> LookupSpan<'span>>()
-> Result<impl Layer<S>, TraceError> {
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(OTEL_EXPORTER_OTLP_ENDPOINT.clone()),
        )
        .with_trace_config(
            trace::Config::default().with_resource(Resource::new([KeyValue {
                key: Key::from_static_str("service.name"),
                value: Value::from("bnv-manager"),
            }])),
        )
        .install_batch(runtime::Tokio)?;

    let tracer = provider.tracer("bnv-manager");

    Ok(tracing_opentelemetry::layer()
        .with_threads(false) // It's a tokio worker anyway
        .with_tracked_inactivity(false)
        .with_tracer(tracer))
}
