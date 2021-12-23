use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoRequest {
    pub team: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub team: Option<Team>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Team {
    pub id: Option<String>,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub email_domain: Option<String>,
    pub icon: Option<Icon>,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
}

// TODO
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Icon {
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_default: Option<bool>,
}

pub async fn info<T>(
    client: &T,
    param: &InfoRequest,
    bot_token: &str,
) -> Result<InfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.info");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
