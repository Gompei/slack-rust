use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PreferencesListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub allow_message_deletion: Option<bool>,
    pub display_real_names: Option<bool>,
    pub disable_file_uploads: Option<String>,
    pub msg_edit_window_mins: Option<i32>,
    pub who_can_post_general: Option<String>,
}

pub async fn preferences_list<T>(
    client: &T,
    bot_token: &str,
) -> Result<PreferencesListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.preferences.list");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<PreferencesListResponse>(&result).map_err(Error::SerdeJsonError)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_response() {
        let response = PreferencesListResponse {
            ok: true,
            allow_message_deletion: Some(true),
            display_real_names: Some(false),
            disable_file_uploads: Some("disable_all".to_string()),
            msg_edit_window_mins: Some(25),
            who_can_post_general: Some("everyone".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "allow_message_deletion": true,
  "display_real_names": false,
  "disable_file_uploads": "disable_all",
  "msg_edit_window_mins": 25,
  "who_can_post_general": "everyone"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<PreferencesListResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_info() {
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post().returning(|_, _| {
            Ok(r##"{
  "ok": true,
  "allow_message_deletion": true,
  "display_real_names": false,
  "disable_file_uploads": "disable_all",
  "msg_edit_window_mins": 25,
  "who_can_post_general": "everyone"
}"##
            .to_string())
        });

        let response = preferences_list(&mock, &"test_token".to_string())
            .await
            .unwrap();
        let expect = PreferencesListResponse {
            ok: true,
            allow_message_deletion: Some(true),
            display_real_names: Some(false),
            disable_file_uploads: Some("disable_all".to_string()),
            msg_edit_window_mins: Some(25),
            who_can_post_general: Some("everyone".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
