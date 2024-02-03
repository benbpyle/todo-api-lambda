use std::env;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use opentelemetry::{global, KeyValue};
use opentelemetry::trace::Tracer;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{Resource, trace};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Deserialize, Debug)]
struct Request {
}

#[derive(Serialize)]
struct Response {
    status_code: i32,
    body: String,
}

#[instrument(name = "handler")]
async fn function_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    info!("{:?}", "An Info Message".to_string());

    // Prepare the response
    let resp = Response {
        status_code: 200,
        body: "Hello World!".to_string(),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317")
        .with_protocol(Protocol::Grpc);

    let tracer =    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(
            trace::config()
                .with_resource(Resource::new(vec![KeyValue::new("service.name", "lambda-app")])),
        )
        .install_simple()?;


    let telemetry_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer);

    let fmt_layer = fmt::layer()
        .with_target(false) // don't include event targets when logging
        .pretty()
        .json();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(telemetry_layer)
        .with(EnvFilter::from_env("APP_LOG"))
        .init();

    run(service_fn(function_handler)).await
}