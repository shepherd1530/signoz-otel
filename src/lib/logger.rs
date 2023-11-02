use opentelemetry::sdk::trace as sdktrace;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use std::env;
use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;

use tracing_subscriber::{filter, Layer, Registry};

fn init_tracer() -> opentelemetry_sdk::trace::Tracer {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
        .with_trace_config(sdktrace::config().with_resource(Resource::default()))
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap()
}

pub fn get_subscriber(
    name: String,
    env_filter: String,
) -> (impl Subscriber + Send + Sync, WorkerGuard) {
    let env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    let env_filter = filter::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| filter::EnvFilter::new(env_filter));

    let file_appender = tracing_appender::rolling::daily("./", "web.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = BunyanFormattingLayer::new(name, non_blocking)
        .with_filter(filter_fn(move |_| env == "production"));

    let pretty_formatting_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(filter_fn(move |_| true));

    let logger = Registry::default()
        .with(env_filter)
        .with(OpenTelemetryLayer::new(init_tracer()))
        .with(pretty_formatting_layer)
        .with(file_layer);

    (logger, guard)
}

pub fn init_subscriber(subscriber: (impl Subscriber + Send + Sync, WorkerGuard)) -> WorkerGuard {
    let guard = subscriber.1;

    tracing::subscriber::set_global_default(subscriber.0).unwrap();

    guard
}
