//! Retrieve a permalink URL for a specific extant message.  

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetPermalinkRequest {
    pub channel: String,
    pub message_ts: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetPermalinkResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub permalink: Option<String>,
}

/// Retrieve a permalink URL for a specific extant message.  
/// See: <https://api.slack.com/methods/chat.getPermalink>
pub async fn get_permalink<T>(
    client: &T,
    param: &GetPermalinkRequest,
    bot_token: &str,
) -> Result<GetPermalinkResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.getPermalink");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<GetPermalinkResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = GetPermalinkRequest {
            channel: "53072".to_string(),
            message_ts: "1234567890.123456".to_string(),
        };
        let json = r##"{
  "channel": "53072",
  "message_ts": "1234567890.123456"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<GetPermalinkRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = GetPermalinkResponse {
            ok: true,
            permalink: Some(
                "https://ghostbusters.slack.com/archives/C1H9RESGA/p135854651500008".to_string(),
            ),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "permalink": "https://ghostbusters.slack.com/archives/C1H9RESGA/p135854651500008"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<GetPermalinkResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_get_permalink() {
        let param = GetPermalinkRequest {
            channel: "53072".to_string(),
            message_ts: "1234567890.123456".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "permalink": "https://ghostbusters.slack.com/archives/C1H9RESGA/p135854651500008"
}"##
            .to_string())
        });

        let response = get_permalink(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = GetPermalinkResponse {
            ok: true,
            permalink: Some(
                "https://ghostbusters.slack.com/archives/C1H9RESGA/p135854651500008".to_string(),
            ),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
