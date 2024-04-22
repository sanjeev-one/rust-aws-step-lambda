use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use aws_lambda_events::event::s3::S3Event;
use log::{LevelFilter, info};
use simple_logger::SimpleLogger;
use aws_sdk_s3::Client as S3Client;
use aws_config::meta::region::RegionProviderChain;

async fn read_file(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let s3_event: S3Event = serde_json::from_value(event)?;
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let s3_client = S3Client::new(&config);

    for record in s3_event.records {
        let bucket_name = record.s3.bucket.name.unwrap_or_default();
        let key = record.s3.object.key.unwrap_or_default();

        // Download the file content
        let resp = s3_client.get_object().bucket(&bucket_name).key(&key).send().await?;
        let data = resp.body.collect().await?;
        let contents = String::from_utf8_lossy(&data.into_bytes());

        // Count words
        let word_count = contents.split_whitespace().count();
        info!("Processed file: {}/{} with word count: {}", bucket_name, key, word_count);

        return Ok(json!({"file_name": key, "word_count": word_count}));
    }
    Ok(json!({}))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    lambda_runtime::run(service_fn(read_file)).await?;
    Ok(())
}
