/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use aws_sdk_dynamodb::{Client, Config, Region};
use aws_sdk_dynamodb::model::AttributeValue;
use chrono::prelude::*;

use crate::database::Error;
use crate::REGION;

pub struct LoginsTable {}

impl LoginsTable {
    fn client() -> Client {
        let shared_config = Config::builder()
            .region(Region::new(REGION))
            .build();
        Client::from_conf(shared_config)
    }
}

impl LoginsTable {
    pub async fn record_login(user_id: &str) -> Result<(), Error> {
        match Self::client()
            .put_item()
            .table_name(TABLE_NAME)
            .item(USER_ID_ATTRIBUTE, AttributeValue::S(user_id.to_string()))
            .item(TIMESTAMP_ATTRIBUTE, AttributeValue::S(Utc::now().to_string()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::PutItem(e))
        }
    }
}

const TABLE_NAME: &str = "logins";
const USER_ID_ATTRIBUTE: &str = "userId";
const TIMESTAMP_ATTRIBUTE: &str = "timestamp";
