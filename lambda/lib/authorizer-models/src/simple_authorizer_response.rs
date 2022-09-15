/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SimpleAuthorizerResponse {
    isAuthorized: bool,
    context: HashMap<String, String>,
}

impl SimpleAuthorizerResponse {

    pub fn new(is_authorized: bool, context: HashMap<String, String>) -> SimpleAuthorizerResponse {
        SimpleAuthorizerResponse {
            isAuthorized: is_authorized,
            context,
        }
    }
}

impl SimpleAuthorizerResponse {

    pub fn is_authorized(&self) -> bool {
        self.isAuthorized
    }
    pub fn context(&self) -> &HashMap<String, String> {
        &self.context
    }

    pub fn set_is_authorized(&mut self, is_authorized: bool) {
        self.isAuthorized = is_authorized;
    }
    pub fn set_context(&mut self, context: HashMap<String, String>) {
        self.context = context;
    }

    pub fn insert_context_var(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }
}
