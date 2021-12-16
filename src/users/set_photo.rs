use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, ResponseMetadata, SlackWebAPIClient};
use crate::users::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SetPhotoRequest {
    pub crop_w: Option<String>,
    pub crop_x: Option<String>,
    pub crop_y: Option<String>,
    // TOOD
    pub image: Option<String>,
}

// TODO
pub async fn set_photo<T>(
    client: &T,
    param: &SetPhotoRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.setPhoto");
    let json = serde_json::to_string(&param)?;

    client
        .post_multipart_forms(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
