use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, post, Client};

#[derive(Deserialize, Serialize, Debug)]
pub struct ConnectionsOpenResponse {
    pub ok: bool,
    pub url: Option<String>,
    pub error: Option<String>,
}

pub async fn connections_open(
    client: Client,
    app_token: String,
) -> Result<ConnectionsOpenResponse, Error> {
    let url = get_slack_url("apps.connections.open");

    post(client, url, app_token)
        .await?
        .body_json()
        .await
        .map_err(Error::SurfError)
}
