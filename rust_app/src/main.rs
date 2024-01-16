use std::env;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use opentelemetry::{global, KeyValue};
use opentelemetry::trace::Tracer;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{Resource, trace};
use opentelemetry_sdk::runtime::Tokio;

use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};
use tracing::field::debug;
use tracing_subscriber::{EnvFilter, fmt, Layer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize, Debug)]
struct Request {
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
#[instrument(name = "handler")]
async fn function_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    // info!("{:?}", "An Info Message".to_string());

    // error!("{:?}", env::var("RUST_LOG"));
    // error!("{:?}", env::var("SOME_VAR"));
    // Prepare the response
    let resp = Response {
        statusCode: 200,
        body: "Hello World!".to_string(),
    };

    //tracing::Span::current().record("result", "result");
    debug!("{:?}", "A Debug Message".to_string());
    debug!("{:?}", env::var("APP_LOG"));

    // let tracer = global::tracer("global_tracer");
    // // let parent_cx = global::get_text_map_propagator(|propagator| {
    // //     propagator.extract()
    // // });
    // tracer.start("SomeRun");

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
        .install_batch(Tokio)?;


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