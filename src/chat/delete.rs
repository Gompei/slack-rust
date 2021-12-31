//! Deletes a message.  
//! See: <https://api.slack.com/methods/chat.delete>

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DeleteRequest {
    pub channel: String,
    pub ts: String,
    pub as_user: Option<bool>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DeleteResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub channel: Option<String>,
    pub ts: Option<String>,
}

/// Deletes a message.
/// See: <https://api.slack.com/methods/chat.delete>
pub async fn delete<T>(
    client: &T,
    param: &DeleteRequest,
    bot_token: &str,
) -> Result<DeleteResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.delete");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = DeleteRequest {
            channel: "C1234567890".to_string(),
            ts: "1405894322.002768".to_string(),
            as_user: Some(true),
        };
        let json = r##"{
  "channel": "C1234567890",
  "ts": "1405894322.002768",
  "as_user": true
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<DeleteRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = DeleteResponse {
            ok: true,
            channel: Some("C1234567890".to_string()),
            ts: Some("1405894322.002768".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "channel": "C1234567890",
  "ts": "1405894322.002768"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<DeleteResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_delete() {
        let param = DeleteRequest {
            channel: "C1234567890".to_string(),
            ts: "1405894322.002768".to_string(),
            as_user: Some(true),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "channel": "C1234567890",
  "ts": "1405894322.002768"
}"##
            .to_string())
        });

        let response = delete(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DeleteResponse {
            ok: true,
            channel: Some("C1234567890".to_string()),
            ts: Some("1405894322.002768".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
