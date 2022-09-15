/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use std::collections::HashMap;

use lambda_runtime::{Error, LambdaEvent, run, service_fn};

use authorizer_models::{SimpleAuthorizerRequest, SimpleAuthorizerResponse};
use sign_in_with_apple::validate;

use crate::values::{AUTHORIZATION_HEADER, DEVICE_ID_HEADER, TOKEN_PREFIX, USER_ID_HEADER};

mod values;

async fn handler(event: LambdaEvent<SimpleAuthorizerRequest>) -> Result<SimpleAuthorizerResponse, Error> {
    let mut context = HashMap::new();

    // get headers
    let headers = event.payload.headers();
    let authorization;
    match headers.get(AUTHORIZATION_HEADER) {
        Some(value) => {
            match value.strip_prefix(TOKEN_PREFIX) {
                Some(v) => authorization = String::from(v),
                None => {
                    context.insert(String::from("failure"), String::from("Invalid Authorization header"));
                    return Ok(SimpleAuthorizerResponse::new(false, context));
                }
            }
        }
        None => {
            context.insert(String::from("failure"), String::from("Missing Authorization header"));
            return Ok(SimpleAuthorizerResponse::new(false, context));
        }
    }
    let user_id;
    match headers.get(USER_ID_HEADER) {
        Some(value) => user_id = String::from(value),
        None => {
            context.insert(String::from("failure"), String::from("Missing UserId header"));
            return Ok(SimpleAuthorizerResponse::new(false, context));
        }
    }
    let device_id;
    match headers.get(DEVICE_ID_HEADER) {
        Some(value) => device_id = String::from(value),
        None => {
            context.insert(String::from("failure"), String::from("Missing DeviceId header"));
            return Ok(SimpleAuthorizerResponse::new(false, context));
        }
    }

    // validate
    match validate(user_id, authorization, device_id, false).await {
        Ok(_) => Ok(SimpleAuthorizerResponse::new(true, context)),
        Err(e) => {
            context.insert(String::from("failure"), format!("Token validation error: {e}"));
            Ok(SimpleAuthorizerResponse::new(false, context))
        }
    }
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

#[cfg(test)]
mod test {
    use lambda_runtime::{Context, LambdaEvent};

    use crate::{handler, SimpleAuthorizerRequest};

    #[tokio::test]
    async fn test_missing_header() {
        let input_str = include_str!("../tests/missing_header.json");
        let payload: SimpleAuthorizerRequest = serde_json::from_str(input_str).expect("Failed to deserialize request");
        let request = LambdaEvent::new(payload, Context::default());
        let response = handler(request).await.expect("Failed to handle request");
        assert!(!response.is_authorized());
        assert_eq!(response.context().len(), 1);
        assert_eq!(response.context().get("failure").expect("Missing context variable"), "Missing Authorization header");
    }

    #[tokio::test]
    async fn test_not_allowed() {
        let input_str = include_str!("../tests/not_allowed.json");
        let payload: SimpleAuthorizerRequest = serde_json::from_str(input_str).expect("Failed to deserialize request");
        let request = LambdaEvent::new(payload, Context::default());
        let response = handler(request).await.expect("Failed to handle request");
        assert!(!response.is_authorized());
        assert_eq!(response.context().len(), 1);
        assert_eq!(response.context().get("failure").expect("Missing context variable"), "Missing Authorization header");
    }

    #[tokio::test]
    async fn test_has_auth_header() {
        let input_str = include_str!("../tests/has_auth_header.json");
        let payload: SimpleAuthorizerRequest = serde_json::from_str(input_str).expect("Failed to deserialize request");
        let request = LambdaEvent::new(payload, Context::default());
        let response = handler(request).await.expect("Failed to handle request");
        assert!(!response.is_authorized());
        assert_eq!(response.context().len(), 1);
        assert_eq!(response.context().get("failure").expect("Missing context variable"), "Missing UserId header");
    }

    #[tokio::test]
    async fn test_real_input() {
        let input_str = include_str!("../tests/real_input.json");
        let payload: SimpleAuthorizerRequest = serde_json::from_str(input_str).expect("Failed to deserialize request");
        let request = LambdaEvent::new(payload, Context::default());
        let response = handler(request).await.expect("Failed to handle request");
        assert!(!response.is_authorized());
        assert_eq!(response.context().len(), 1);
        assert_eq!(response.context().get("failure").expect("Missing context variable"), "Missing Authorization header");
    }
}