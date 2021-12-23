use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ConnectionsOpenResponse {
    pub ok: bool,
    pub url: Option<String>,
    pub error: Option<String>,
}

pub async fn connections_open<T>(
    client: &T,
    app_token: &str,
) -> Result<ConnectionsOpenResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("apps.connections.open");

    client.post(&url, app_token).await.and_then(|result| {
        serde_json::from_str::<ConnectionsOpenResponse>(&result).map_err(Error::SerdeJsonError)
    })
}
