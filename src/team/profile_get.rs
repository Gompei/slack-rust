use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProfileGetRequest {
    pub visibility: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProfileGetResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub profile: Option<Profile>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Profile {
    pub fields: Option<Vec<Field>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Field {
    pub id: Option<String>,
    pub ordering: Option<i8>,
    pub label: Option<String>,
    pub hint: Option<String>,
    pub r#type: Option<String>,
    pub possible_values: Option<Vec<String>>,
    pub options: Option<Options>,
    pub is_hidden: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Options {
    pub is_protected: Option<i8>,
}

pub async fn profile_get<T>(
    client: &T,
    param: &ProfileGetRequest,
    bot_token: &str,
) -> Result<ProfileGetResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.profile.get");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ProfileGetResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
