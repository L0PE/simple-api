use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use aws_config;
use aws_sdk_dynamodb;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let lambda_handler_function = service_fn(handler);
    lambda_runtime::run(lambda_handler_function).await?;
    Ok(())
}

async fn handler(_lambda_event: LambdaEvent<Value>) -> Result<Value, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    let dynamo_db_response = client
        .scan()
        .table_name(env::var("DYNAMO_DB_TABLE_NAME").unwrap())
        .limit(20)
        .send()
        .await?;

    let (response_body, code) = match dynamo_db_response.items() {
        Some(messages) => (
            messages.iter()
                .map(|message| json!({
                "id": message.get("id").unwrap().as_s().unwrap(),
                "username": message.get("username").unwrap().as_s().unwrap(),
                "message": message.get("message").unwrap().as_s().unwrap(),
                "created_at": message.get("created_at").unwrap().as_s().unwrap()
            }))
                .collect(),
            200
        ),
        _ => (json!({"message": "Something went wrong!"}), 500)
    };

    Ok(json!({
        "statusCode": code,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": format!("{}", response_body),
        "isBase64Encoded": false
    }))
}
