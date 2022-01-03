//! Sets the purpose for a conversation.

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SetPurposeRequest {
    pub channel: String,
    pub purpose: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SetPurposeResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub purpose: Option<String>,
}

/// Sets the purpose for a conversation.  
/// See: <https://api.slack.com/methods/conversations.setPurpose>
pub async fn set_purpose<T>(
    client: &T,
    param: &SetPurposeRequest,
    bot_token: &str,
) -> Result<SetPurposeResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.setPurpose");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<SetPurposeResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = SetPurposeRequest {
            channel: "C1234567890".to_string(),
            purpose: "My More Special Purpose".to_string(),
        };
        let json = r##"{
  "channel": "C1234567890",
  "purpose": "My More Special Purpose"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<SetPurposeRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = SetPurposeResponse {
            ok: true,
            purpose: Some("I didn't set this purpose on purpose!".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "purpose": "I didn't set this purpose on purpose!"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<SetPurposeResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_set_purpose() {
        let param = SetPurposeRequest {
            channel: "C1234567890".to_string(),
            purpose: "My More Special Purpose".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "purpose": "I didn't set this purpose on purpose!"
}"##
            .to_string())
        });

        let response = set_purpose(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = SetPurposeResponse {
            ok: true,
            purpose: Some("I didn't set this purpose on purpose!".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
