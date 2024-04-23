use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    input_string: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
use std::collections::HashMap;

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract the input string from the request

    info!("Received a payload!!!");

    let input_string = event.payload.input_string;

    // Calculate letter frequency
    let letter_frequency = calculate_letter_frequency(&input_string);

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Letter frequency: {:?}", letter_frequency),
    };

    info!("Processed the payload : {} !!!", input_string);

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

fn calculate_letter_frequency(input: &str) -> HashMap<char, usize> {
    let mut frequency_map: HashMap<char, usize> = HashMap::new();

    for c in input.chars() {
        if c.is_alphabetic() {
            *frequency_map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }
    frequency_map
}

// Update the main function to reflect the changes:

//rust
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
