/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use lambda_http::{Body, Error, Request, Response, run, service_fn};
use snipsnap_lib::database::LoginsTable;
use snipsnap_lib::http::{HttpErrorResponse, HttpResponseGenerator};

use crate::response::NonceResetResponse;

mod response;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // get userId from event body
    // record in database
    // return
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
