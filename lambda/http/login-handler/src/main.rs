/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use lambda_http::{Body, Error, Request, Response, run, service_fn};
use serde::Serialize;
use snipsnap_lib::database::LoginsTable;
use snipsnap_lib::http::{HttpErrorResponse, HttpResponseGenerator};

#[derive(Serialize)]
struct LoginResponse {
    message: String,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    if let Some(user_id_header) = event.headers().get("X-UserId") {
        if let Ok(user_id) = user_id_header.to_str() {
            return match LoginsTable::record_login(user_id).await {
                Ok(_) => {
                    let body = LoginResponse { message: "Login successful and logged!".to_string() };
                    HttpResponseGenerator::response(200, &body)
                },
                Err(e) => {
                    let body = HttpErrorResponse::new(format!("Error recording login: {e}"));
                    HttpResponseGenerator::response(500, &body)
                }
            }
        }
    }

    let body = HttpErrorResponse::new("Could not get X-UserId header".to_string());
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

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test() {
        assert!(true)
    }
}
