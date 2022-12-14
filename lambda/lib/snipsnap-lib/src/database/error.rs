/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

use aws_sdk_dynamodb::error::{DeleteItemError, PutItemError, QueryError, UpdateItemError};
use aws_sdk_dynamodb::types::SdkError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to execute query")]
    Query(#[from] SdkError<QueryError>),
    #[error("Failed to put item")]
    PutItem(#[from] SdkError<PutItemError>),
    #[error("Failed to update item")]
    UpdateItem(#[from] SdkError<UpdateItemError>),
    #[error("Failed to delete item")]
    DeleteItem(#[from] SdkError<DeleteItemError>),
    #[error("Item not found")]
    NotFound,
    #[error("Some item(s) were found but there was an error retrieving attributes")]
    AttributeError
}
