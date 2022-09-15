/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use std::collections::HashMap;
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct SimpleAuthorizerRequest {

    version: String,
    #[serde(rename = "type")]
    payload_type: String,
    routeArn: String,
    identitySource: Value,
    routeKey: String,
    rawPath: String,
    rawQueryString: String,
    #[serde(default = "Vec::new")]
    cookies: Vec<String>,
    #[serde(default = "HashMap::new")]
    headers: HashMap<String, String>,
    #[serde(default = "HashMap::new")]
    queryStringParameters: HashMap<String, String>,
    // would not expect this to be missing
    requestContext: Map<String, Value>,
    #[serde(default = "HashMap::new")]
    pathParameters: HashMap<String, String>,
    #[serde(default = "HashMap::new")]
    stageVariables: HashMap<String, String>,
}

impl SimpleAuthorizerRequest {

    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn payload_type(&self) -> &str {
        &self.payload_type
    }
    pub fn route_arn(&self) -> &str {
        &self.routeArn
    }
    pub fn identity_source(&self) -> &Value {
        &self.identitySource
    }
    pub fn route_key(&self) -> &str {
        &self.routeKey
    }
    pub fn raw_path(&self) -> &str {
        &self.rawPath
    }
    pub fn raw_query_string(&self) -> &str {
        &self.rawQueryString
    }
    pub fn cookies(&self) -> &Vec<String> {
        &self.cookies
    }
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    pub fn query_string_parameters(&self) -> &HashMap<String, String> {
        &self.queryStringParameters
    }
    pub fn request_context(&self) -> &Map<String, Value> {
        &self.requestContext
    }
    pub fn path_parameters(&self) -> &HashMap<String, String> {
        &self.pathParameters
    }
    pub fn stage_variables(&self) -> &HashMap<String, String> {
        &self.stageVariables
    }
}
