/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use lambda_http::{Body, Error, Request, Response, run, service_fn};
use serde::{Deserialize, Serialize};
use snipsnap_lib::database::NoncesTable;
use snipsnap_lib::http::{HttpErrorResponse, HttpResponseGenerator};

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct GetNonceRequest {
    deviceId: String,
}

#[derive(Serialize)]
struct GetNonceResponse {
    nonce: String,
}

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    if let Ok(decoded) = String::from_utf8(event.body().to_vec()) {
        if let Ok(deserialized) = serde_json::from_str::<GetNonceRequest>(&decoded) {
            return match NoncesTable::make_nonce(&*deserialized.deviceId).await {
                Ok(nonce) => {
                    let body = GetNonceResponse { nonce };
                    HttpResponseGenerator::response(200, &body)
                },
                Err(e) => {
                    let body = HttpErrorResponse::new(format!("Error making nonce: {e}"));
                    HttpResponseGenerator::response(500, &body)
                }
            }
        }
    }

    let body = HttpErrorResponse::new("Error decoding body".to_string());
    HttpResponseGenerator::response(400, &body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(handler)).await
}
