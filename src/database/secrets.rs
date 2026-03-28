use aws_config::{self, BehaviorVersion, Region};
use aws_sdk_secretsmanager;

#[::tokio::main]
async fn main() -> Result<(), aws_sdk_secretsmanager::Error> {
    let secret_name = "Temperature_DB";
    let region = Region::new("us-east-2");

    let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        .region(region)
        .load()
        .await;

    let asm = aws_sdk_secretsmanager::Client::new(&config);

    let response = asm
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;
    // For a list of exceptions thrown, see
    // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html

    let secret_string = response.secret_string();

    // Your code goes here

    Ok(())
}