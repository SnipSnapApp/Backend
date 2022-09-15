/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */


mod simple_authorizer_response;
mod simple_authorizer_request;

pub use simple_authorizer_response::SimpleAuthorizerResponse;
pub use simple_authorizer_request::SimpleAuthorizerRequest;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::{File, read_to_string};
    use serde_json::from_reader;
    use serde_json::error::Result;
    use serde_json::Value::{Array, Null};
    use crate::simple_authorizer_request::SimpleAuthorizerRequest;
    use crate::simple_authorizer_response::SimpleAuthorizerResponse;

    #[test]
    fn test_invalid_request() {
        let result: Result<SimpleAuthorizerRequest> = from_reader(File::open("tests/invalid_request.json").expect("expected file"));
        assert!(result.is_err())
    }

    #[test]
    fn test_valid_full_request() {
        let result: Result<SimpleAuthorizerRequest> = from_reader(File::open("tests/valid_full_request.json").expect("expected file"));
        assert!(result.is_ok());
        let request = result.expect("Error getting request from result");

        assert_eq!(request.version(), "2.0", "version");
        assert_eq!(request.payload_type(), "REQUEST", "type");
        assert_eq!(request.route_arn(), "arn:aws:execute-api:us-east-1:123456789012:abcdef123/test/GET/request", "route arn");
        if let Array(identity_source) = request.identity_source() {
            assert_eq!(identity_source.len(), 2, "identity source length");
            assert_eq!(identity_source.get(0).expect("Vec error"), "user1", "identity source[0]")
        } else {
            panic!("identity source was not an Array Value");
        }
        assert_eq!(request.route_key(), "$default", "route key");
        assert_eq!(request.raw_path(), "/my/path", "raw path");
        assert_eq!(request.raw_query_string(), "parameter1=value1&parameter1=value2&parameter2=value", "raw query string");
        assert_eq!(request.cookies().len(), 2, "cookies length");
        assert_eq!(request.cookies().get(0).expect("Vec error"), "cookie1", "cookies[0]");
        assert_eq!(request.headers().len(), 3, "headers length");
        assert_eq!(request.headers().get("Header1").expect("HashMap error"), "value1", "headers[Header1]");
        assert_eq!(request.query_string_parameters().len(), 2, "query string parameters length");
        assert_eq!(request.query_string_parameters().get("parameter1").expect("Vec error"), "value1,value2", "query string parameters[parameter1]");
        // no tests on request context
        assert_eq!(request.path_parameters().len(), 1, "path parameters length");
        assert_eq!(request.path_parameters().get("parameter1").expect("HashMap error"), "value1", "path parameters[parameter1]");
        assert_eq!(request.stage_variables().len(), 2, "stage variables length");
        assert_eq!(request.stage_variables().get("stageVariable1").expect("HashMap error"), "value1", "stage variables[stageVariable1]");
    }

    #[test]
    fn test_valid_short_request() {
        let result: Result<SimpleAuthorizerRequest> = from_reader(File::open("tests/valid_short_request.json").expect("expected file"));
        assert!(result.is_ok());
        let request = result.expect("Error getting request from result");

        assert_eq!(request.version(), "2.0", "version");
        assert_eq!(request.payload_type(), "REQUEST", "type");
        assert_eq!(request.route_arn(), "arn:aws:execute-api:us-west-2:174026058454:7osxrt1c19/$default/POST/test2", "route arn");
        if let Null = request.identity_source() {
        } else {
            panic!("identity source was not a Null Value");
        }
        assert_eq!(request.route_key(), "POST /test2", "route key");
        assert_eq!(request.raw_path(), "/test2", "raw path");
        assert_eq!(request.raw_query_string(), "", "raw query string");
        assert!(request.cookies().is_empty(), "cookies length");
        assert_eq!(request.headers().len(), 11, "headers length");
        assert_eq!(request.headers().get("accept").expect("HashMap error"), "*/*", "headers[accept]");
        assert!(request.query_string_parameters().is_empty(), "query string parameters length");
        // no tests on request context
        assert!(request.path_parameters().is_empty(), "path parameters length");
        assert!(request.stage_variables().is_empty(), "stage variables length");
    }

    #[test]
    fn test_valid_response() {
        let is_authorized = true;
        let mut context = HashMap::new();
        context.insert(String::from("exampleKey"), String::from("exampleValue"));
        let response = SimpleAuthorizerResponse::new(is_authorized, context);
        let mut serialized = serde_json::to_string(&response).expect("Failed to serialize response");
        let mut expected = read_to_string("tests/valid_response.json").expect("Expected file");
        remove_whitespace(&mut serialized);
        remove_whitespace(&mut expected);
        assert_eq!(serialized, expected, "Response was not properly serialized");
    }

    fn remove_whitespace(s: &mut String) {
        s.retain(|c| !c.is_whitespace());
    }
}
