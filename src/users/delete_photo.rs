use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, ResponseMetadata, SlackWebAPIClient};

pub async fn delete_photo<T>(client: &T, bot_token: &str) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.deletePhoto");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
    })
}
