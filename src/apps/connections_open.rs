use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct ConnectionsOpenResponse {
    pub ok: bool,
    pub url: Option<String>,
    pub error: Option<String>,
}

pub async fn connections_open<T>(
    client: &T,
    app_token: String,
) -> Result<ConnectionsOpenResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("apps.connections.open");

    client
        .post(url, app_token)
        .await?
        .body_json()
        .await
        .map_err(Error::SurfError)
}
