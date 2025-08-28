use aws_sdk_s3::Client;
use aws_config::meta::region::RegionProviderChain;
use aws_types::region::Region;

pub async fn get_s3_client() -> Client {
    let region_provider = RegionProviderChain::first_try(
        std::env::var("AWS_REGION").ok().map(Region::new)
    )
    .or_default_provider()
    .or_else(Region::new("ap-northeast-1"));

    let config = aws_config::from_env()
        .region(region_provider)
        .load()
        .await;
    Client::new(&config)
}