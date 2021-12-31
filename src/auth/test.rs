//! Checks authentication & identity.
//! See: <https://api.slack.com/methods/auth.test>

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TestResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub url: Option<String>,
    pub team: Option<String>,
    pub user: Option<String>,
    pub team_id: Option<String>,
    pub user_id: Option<String>,
}

/// Checks authentication & identity.
/// See: <https://api.slack.com/methods/auth.test>
pub async fn test<T>(client: &T, bot_token: &str) -> Result<TestResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("auth.test");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<TestResponse>(&result).map_err(Error::SerdeJsonError)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_response() {
        let response = TestResponse {
            ok: true,
            url: Some("https://subarachnoid.slack.com/".to_string()),
            team: Some("Subarachnoid Workspace".to_string()),
            user: Some("grace".to_string()),
            team_id: Some("T12345678".to_string()),
            user_id: Some("W12345678".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "url": "https://subarachnoid.slack.com/",
  "team": "Subarachnoid Workspace",
  "user": "grace",
  "team_id": "T12345678",
  "user_id": "W12345678"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<TestResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_auth_test() {
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post().returning(|_, _| {
            Ok(r##"{
          "ok": true,
          "url": "https://subarachnoid.slack.com/",
          "team": "Subarachnoid Workspace",
          "user": "grace",
          "team_id": "T12345678",
          "user_id": "W12345678"  
        }"##
            .to_string())
        });

        let response = test(&mock, &"test_token".to_string()).await.unwrap();
        let expect = TestResponse {
            ok: true,
            url: Some("https://subarachnoid.slack.com/".to_string()),
            team: Some("Subarachnoid Workspace".to_string()),
            user: Some("grace".to_string()),
            team_id: Some("T12345678".to_string()),
            user_id: Some("W12345678".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
