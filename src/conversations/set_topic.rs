//! Sets the topic for a conversation.

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SetTopicRequest {
    pub channel: String,
    pub topic: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SetTopicResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub topic: Option<String>,
}

/// Sets the topic for a conversation.  
/// See: <https://api.slack.com/methods/conversations.setTopic>
pub async fn set_topic<T>(
    client: &T,
    param: &SetTopicRequest,
    bot_token: &str,
) -> Result<SetTopicResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.setTopic");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<SetTopicResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = SetTopicRequest {
            channel: "C1234567890".to_string(),
            topic: "Apply topically for best effects".to_string(),
        };
        let json = r##"{
  "channel": "C1234567890",
  "topic": "Apply topically for best effects"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<SetTopicRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = SetTopicResponse {
            ok: true,
            topic: Some("Apply topically for best effects".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "topic": "Apply topically for best effects"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<SetTopicResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_set_topic() {
        let param = SetTopicRequest {
            channel: "C1234567890".to_string(),
            topic: "Apply topically for best effects".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "topic": "Apply topically for best effects"
}"##
            .to_string())
        });

        let response = set_topic(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = SetTopicResponse {
            ok: true,
            topic: Some("Apply topically for best effects".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
