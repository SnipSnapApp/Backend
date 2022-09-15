/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use aws_sdk_dynamodb::{Client, Config, Region};
use aws_sdk_dynamodb::model::{AttributeDefinition, AttributeValue, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType, Select};
use aws_sdk_dynamodb::output::{CreateTableOutput, DeleteTableOutput};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::database::Error;
use crate::REGION;

pub struct NoncesTable {}

impl NoncesTable {
    fn client() -> Client {
        let shared_config = Config::builder()
            .region(Region::new(REGION))
            .build();
        Client::from_conf(shared_config)
    }
}

// nonce functionality
impl NoncesTable {
    pub async fn make_nonce(device_id: &str) -> Result<String, Error> {
        let nonce: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(NONCE_LENGTH)
            .map(char::from)
            .collect();

        let client = Self::client();

        match client.query()
            .table_name(TABLE_NAME)
            .key_condition_expression("#key = :value")
            .expression_attribute_names("#key", DEVICE_ID_ATTRIBUTE)
            .expression_attribute_values(":value", AttributeValue::S(String::from(device_id)))
            .select(Select::AllAttributes)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.count > 0 {
                    match client.update_item()
                        .table_name(TABLE_NAME)
                        .key(DEVICE_ID_ATTRIBUTE, AttributeValue::S(String::from(device_id)))
                        .update_expression("SET #key = :value")
                        .expression_attribute_names("#key", NONCE_ATTRIBUTE)
                        .expression_attribute_values(":value", AttributeValue::S(String::from(nonce.clone())))
                        .send()
                        .await
                    {
                        Ok(_) => Ok(nonce),
                        Err(e) => Err(Error::UpdateItem(e))
                    }
                } else {
                    match client.put_item()
                        .table_name(TABLE_NAME)
                        .item(DEVICE_ID_ATTRIBUTE, AttributeValue::S(String::from(device_id)))
                        .item(NONCE_ATTRIBUTE, AttributeValue::S(String::from(nonce.clone())))
                        .send()
                        .await
                    {
                        Ok(_) => Ok(nonce),
                        Err(e) => Err(Error::PutItem(e))
                    }
                }
            }
            Err(e) => Err(Error::Query(e))
        }
    }

    pub async fn get_nonce(device_id: &str) -> Result<String, Error> {
        match Self::read_nonce(device_id).await {
            Ok(nonce) => {
                match Self::client().delete_item()
                    .table_name(TABLE_NAME)
                    .key(DEVICE_ID_ATTRIBUTE, AttributeValue::S(String::from(device_id)))
                    .send()
                    .await
                {
                    Ok(_) => Ok(nonce),
                    Err(e) => Err(Error::DeleteItem(e))
                }
            }
            Err(e) => Err(e)
        }
    }
}

// admin functionality
impl NoncesTable {
    pub async fn reset_nonces() -> Result<(), Error> {
        match Self::delete_table().await {
            Err(e) => return Err(e),
            _ => {}
        }
        match Self::create_table().await {
            Err(e) => return Err(e),
            _ => {}
        }
        Ok(())
    }

    pub async fn read_nonce(device_id: &str) -> Result<String, Error> {
        match Self::client().query()
            .table_name(TABLE_NAME)
            .key_condition_expression("#key = :value")
            .expression_attribute_names("#key", DEVICE_ID_ATTRIBUTE)
            .expression_attribute_values(":value", AttributeValue::S(String::from(device_id)))
            .select(Select::AllAttributes)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.count() > 0 {
                    if let Some(items) = resp.items() {
                        if let Some(first) = items.first() {
                            if let Some(attribute_value) = first.get(NONCE_ATTRIBUTE) {
                                if let AttributeValue::S(nonce) = attribute_value {
                                    return Ok(String::from(nonce))
                                }
                            }
                        }
                    }
                    Err(Error::AttributeError)
                } else {
                    Err(Error::NotFound)
                }
            },
            Err(e) => Err(Error::Query(e))
        }
    }
}

// create and delete table
impl NoncesTable {
    pub async fn create_table() -> Result<CreateTableOutput, Error> {
        let ad = AttributeDefinition::builder()
            .attribute_name(DEVICE_ID_ATTRIBUTE)
            .attribute_type(ScalarAttributeType::S)
            .build();

        let ks = KeySchemaElement::builder()
            .attribute_name(DEVICE_ID_ATTRIBUTE)
            .key_type(KeyType::Hash)
            .build();

        let pt = ProvisionedThroughput::builder()
            .read_capacity_units(READ_CAPACITY_UNITS)
            .write_capacity_units(WRITE_CAPACITY_UNITS)
            .build();

        Self::client().create_table()
            .table_name(TABLE_NAME)
            .key_schema(ks)
            .attribute_definitions(ad)
            .provisioned_throughput(pt)
            .send()
            .await
            .map_err(Error::CreateTable)
    }

    pub async fn delete_table() -> Result<DeleteTableOutput, Error> {
        Self::client().delete_table()
            .table_name(TABLE_NAME)
            .send()
            .await
            .map_err(Error::DeleteTable)
    }
}

// constants
const TABLE_NAME: &str = "nonces";
const DEVICE_ID_ATTRIBUTE: &str = "deviceId";
const NONCE_ATTRIBUTE: &str = "nonce";
const NONCE_LENGTH: usize = 30;
const READ_CAPACITY_UNITS: i64 = 5;
const WRITE_CAPACITY_UNITS: i64 = 5;
