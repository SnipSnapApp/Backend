/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use serde::{Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct HttpErrorResponse {
    timestamp: u128,
    message: String
}

impl HttpErrorResponse {

    pub fn new(message: String) -> HttpErrorResponse {
        HttpErrorResponse {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get current time").as_millis(),
            message
        }
    }
}
