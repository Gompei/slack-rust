//! Deletes a pending scheduled message from the queue.

use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DeleteScheduledMessageRequest {
    pub channel: String,
    pub scheduled_message_id: String,
    pub as_user: Option<bool>,
}

/// Deletes a pending scheduled message from the queue.
/// See: <https://api.slack.com/methods/chat.deleteScheduledMessage>
pub async fn delete_scheduled_message<T>(
    client: &T,
    param: &DeleteScheduledMessageRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.deleteScheduledMessage");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = DeleteScheduledMessageRequest {
            channel: "C123456789".to_string(),
            scheduled_message_id: "Q1234ABCD".to_string(),
            as_user: Some(true),
        };
        let json = r##"{
  "channel": "C123456789",
  "scheduled_message_id": "Q1234ABCD",
  "as_user": true
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<DeleteScheduledMessageRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[async_std::test]
    async fn test_delete_scheduled_message() {
        let param = DeleteScheduledMessageRequest {
            channel: "C123456789".to_string(),
            scheduled_message_id: "Q1234ABCD".to_string(),
            as_user: Some(true),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true
}"##
            .to_string())
        });

        let response = delete_scheduled_message(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
