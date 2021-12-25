use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ScheduledMessagesListRequest {
    pub channel: Option<String>,
    pub cursor: Option<String>,
    pub latest: Option<String>,
    pub limit: Option<i32>,
    pub oldest: Option<String>,
    pub team_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ScheduledMessagesListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub scheduled_messages: Option<Vec<ScheduledMessage>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ScheduledMessage {
    pub id: Option<i32>,
    pub channel_id: Option<String>,
    pub post_at: Option<i32>,
    pub date_created: Option<i32>,
    pub text: Option<String>,
}

pub async fn scheduled_messages_list<T>(
    client: &T,
    param: &ScheduledMessagesListRequest,
    bot_token: &str,
) -> Result<ScheduledMessagesListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.scheduledMessages.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ScheduledMessagesListResponse>(&result)
                .map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::block_object::{TextBlockObject, TextBlockType};
    use crate::block::block_section::SectionBlock;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = ScheduledMessagesListRequest {
            channel: Some("C123456789".to_string()),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            latest: Some("1562137200".to_string()),
            limit: Some(100),
            oldest: Some("1562137200".to_string()),
            team_id: Some("T1234567890".to_string()),
        };
        let json = r##"{
  "channel": "C123456789",
  "cursor": "dXNlcjpVMDYxTkZUVDI=",
  "latest": "1562137200",
  "limit": 100,
  "oldest": "1562137200",
  "team_id": "T1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ScheduledMessagesListRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = ScheduledMessagesListResponse {
            ok: true,
            scheduled_messages: Some(vec![ScheduledMessage {
                id: Some(1298393284),
                channel_id: Some("C1H9RESGL".to_string()),
                post_at: Some(1551991428),
                date_created: Some(1551891734),
                text: Some("Here's a message for you in the future".to_string()),
            }]),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "scheduled_messages": [
    {
      "id": 1298393284,
      "channel_id": "C1H9RESGL",
      "post_at": 1551991428,
      "date_created": 1551891734,
      "text": "Here's a message for you in the future"
    }
  ]
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ScheduledMessagesListResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_scheduled_messages_list() {
        let param = ScheduledMessagesListRequest {
            channel: Some("C123456789".to_string()),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            latest: Some("1562137200".to_string()),
            limit: Some(100),
            oldest: Some("1562137200".to_string()),
            team_id: Some("T1234567890".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "scheduled_messages": [
    {
      "id": 1298393284,
      "channel_id": "C123456789",
      "post_at": 1551991428,
      "date_created": 1551891734,
      "text": "Here's a message for you in the future"
    }
  ]
}"##
            .to_string())
        });

        let response = scheduled_messages_list(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = ScheduledMessagesListResponse {
            ok: true,
            scheduled_messages: Some(vec![ScheduledMessage {
                id: Some(1298393284),
                channel_id: Some("C123456789".to_string()),
                post_at: Some(1551991428),
                date_created: Some(1551891734),
                text: Some("Here's a message for you in the future".to_string()),
            }]),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
