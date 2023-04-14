use std::ops::Add;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use aws_config;
use aws_sdk_dynamodb;
use aws_sdk_dynamodb::types::AttributeValue;
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let lambda_handler_function = service_fn(handler);
    lambda_runtime::run(lambda_handler_function).await?;
    Ok(())
}

async fn handler(lambda_event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _content) = lambda_event.into_parts();
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    validate_event(&event);

    let request = client
        .put_item()
        .table_name(env::var("DYNAMO_DB_TABLE_NAME").unwrap())
        .item("id", AttributeValue::S(Uuid::new_v4().to_string()))
        .item("username", AttributeValue::S(event["username"].to_string()))
        .item("message", AttributeValue::S(event["message"].to_string()))
        .item("created_at", AttributeValue::S(Utc::now().to_string()))
        .item("ttl", AttributeValue::N(Utc::now().add(Duration::hours(1)).timestamp().to_string()));

    request.send().await?;

    Ok(json!({"message": "Message added successfully"}))
}

fn validate_event(event: &Value) {
    assert!(event.is_object());
    assert!(event["username"].is_string());
    assert!(event["message"].is_string());
    assert!(event["message"].to_string().len() < 255)
}