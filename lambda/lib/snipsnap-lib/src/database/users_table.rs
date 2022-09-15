/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use aws_sdk_dynamodb::{Client, Config, Region};
use aws_sdk_dynamodb::model::{AttributeDefinition, AttributeValue, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType, Select};
use aws_sdk_dynamodb::output::{CreateTableOutput, DeleteTableOutput};

use crate::database::error::Error;
use crate::REGION;

pub struct UsersTable {}

// get client
impl UsersTable {
    fn client() -> Client {
        let shared_config = Config::builder()
            .region(Region::new(REGION))
            .build();
        Client::from_conf(shared_config)
    }
}

// login return type
pub struct UserInitInfo {
    pub has_username: bool,
    pub has_display_name: bool
}

// login and logout functionality
impl UsersTable {
    pub async fn login(user_id: &str, sns_arn: &str) -> Result<UserInitInfo, Error> {
        let client = Self::client();

        // check if user exists
        match client.query()
            .table_name(TABLE_NAME)
            .key_condition_expression("#key = :value")
            .expression_attribute_names("#key", USER_ID_ATTRIBUTE)
            .expression_attribute_values(":value", AttributeValue::S(String::from(user_id)))
            .select(Select::AllAttributes)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.count() > 0 {
                    // user exists, update sns_arn
                    match client.update_item()
                        .table_name(TABLE_NAME)
                        .key(USER_ID_ATTRIBUTE, AttributeValue::S(String::from(user_id)))
                        .update_expression("SET #key = :value")
                        .expression_attribute_names("#key", SNS_APRN_ATTRIBUTE)
                        .expression_attribute_values(":value", AttributeValue::S(String::from(sns_arn)))
                        .send()
                        .await
                    {
                        Ok(item) => {
                            // using an expect here instead of a match because it wouldn't make sense for there to be no attributes
                            if let Some(attributes) = item.attributes() {
                                return Ok(UserInitInfo {
                                    has_username: attributes.contains_key(USERNAME_ATTRIBUTE),
                                    has_display_name: attributes.contains_key(DISPLAY_NAME_ATTRIBUTE)
                                })
                            }
                            Err(Error::AttributeError)
                        },
                        Err(e) => Err(Error::UpdateItem(e))
                    }
                } else {
                    // user doesn't exist, create a new entry
                    match client.put_item()
                        .table_name(TABLE_NAME)
                        .item(USER_ID_ATTRIBUTE, AttributeValue::S(String::from(user_id)))
                        .item(SNS_APRN_ATTRIBUTE, AttributeValue::S(String::from(sns_arn)))
                        .send()
                        .await
                    {
                        Ok(_) => Ok(UserInitInfo {
                            has_username: false,
                            has_display_name: false
                        }),
                        Err(e) => Err(Error::PutItem(e))
                    }
                }
            },
            Err(e) => Err(Error::Query(e))
        }
    }

    pub async fn logout(user_id: &str) -> Result<(), Error> {
        match Self::client().update_item()
            .table_name(TABLE_NAME)
            .key(USER_ID_ATTRIBUTE, AttributeValue::S(String::from(user_id)))
            .update_expression("REMOVE #key")
            .expression_attribute_names("#key", SNS_APRN_ATTRIBUTE)
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::UpdateItem(e))
        }
    }
}

// create and delete table
impl UsersTable {
    pub async fn create_table() -> Result<CreateTableOutput, Error> {
        let ad = AttributeDefinition::builder()
            .attribute_name(USER_ID_ATTRIBUTE)
            .attribute_type(ScalarAttributeType::S)
            .build();

        let ks = KeySchemaElement::builder()
            .attribute_name(USER_ID_ATTRIBUTE)
            .key_type(KeyType::Hash)
            .build();

        let pt = ProvisionedThroughput::builder()
            .read_capacity_units(5)
            .write_capacity_units(5)
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
const TABLE_NAME: &str = "users";
const USER_ID_ATTRIBUTE: &str = "userId";
const USERNAME_ATTRIBUTE: &str = "username";
const DISPLAY_NAME_ATTRIBUTE: &str = "displayName";
// const DATE_JOINED_ATTRIBUTE: &str = "dateJoined";
const SNS_APRN_ATTRIBUTE: &str = "snsAprn";
