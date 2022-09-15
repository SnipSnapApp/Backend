/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use serde::Serialize;

#[derive(Serialize)]
pub struct NonceResetResponse {
    pub(crate) success: bool,
}
