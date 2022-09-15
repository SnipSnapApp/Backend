/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use lambda_http::{Body, Error, Response};
use serde::Serialize;
use serde_json;

pub struct HttpResponseGenerator {}

impl HttpResponseGenerator {

    pub fn response<T>(code: u16, body: &T) -> Result<Response<Body>, Error> where T: ?Sized + Serialize  {
        let response = Response::builder()
            .status(code)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&body).expect("").into())
            .map_err(Box::new)?;
        Ok(response)
    }
}
