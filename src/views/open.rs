use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, post_json, Client};
use crate::views::view::View;

#[derive(Deserialize, Serialize, Debug)]
pub struct ViewOpenRequest {
    pub trigger_id: String,
    pub view: View,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ViewOpenResponse {
    pub ok: bool,
    pub view: Option<View>,
}

pub async fn open(
    client: Client,
    param: ViewOpenRequest,
    bot_token: String,
) -> Result<ViewOpenResponse, Error> {
    let url = get_slack_url("views.open");
    let json = serde_json::to_string(&param)?;

    post_json(client, url, json, bot_token)
        .await?
        .body_json()
        .await
        .map_err(Error::SurfError)
}
