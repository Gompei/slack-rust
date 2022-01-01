//! Share a me message into a channel.  

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct MeMessageRequest {
    pub channel: String,
    pub text: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct MeMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub channel: Option<String>,
    pub ts: Option<String>,
}

/// Share a me message into a channel.  
/// See: <https://api.slack.com/methods/chat.meMessage>
pub async fn me_message<T>(
    client: &T,
    param: &MeMessageRequest,
    bot_token: &str,
) -> Result<MeMessageResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.meMessage");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<MeMessageResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = MeMessageRequest {
            channel: "C1234567890".to_string(),
            text: "Hello world".to_string(),
        };
        let json = r##"{
  "channel": "C1234567890",
  "text": "Hello world"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<MeMessageRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = MeMessageResponse {
            ok: true,
            channel: Some("C024BE7LR".to_string()),
            ts: Some("1417671948.000006".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "channel": "C024BE7LR",
  "ts": "1417671948.000006"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<MeMessageResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_me_message() {
        let param = MeMessageRequest {
            channel: "C1234567890".to_string(),
            text: "Hello world".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "channel": "C1234567890",
  "ts": "1417671948.000006"
}"##
            .to_string())
        });

        let response = me_message(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = MeMessageResponse {
            ok: true,
            channel: Some("C1234567890".to_string()),
            ts: Some("1417671948.000006".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
