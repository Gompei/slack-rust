use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct TestResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub url: Option<String>,
    pub team: Option<String>,
    pub user: Option<String>,
    pub team_id: Option<String>,
    pub user_id: Option<String>,
}

pub async fn test<T>(client: &T, bot_token: &str) -> Result<TestResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("auth.test");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<TestResponse>(&result).map_err(Error::SerdeJsonError)
    })
}
