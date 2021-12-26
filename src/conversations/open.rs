use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct OpenRequest {
    pub channel: Option<String>,
    pub prevent_creation: Option<bool>,
    pub return_im: Option<bool>,
    pub users: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct OpenResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub no_op: Option<bool>,
    pub already_open: Option<bool>,
    pub channel: Option<Channel>,
}

pub async fn open<T>(
    client: &T,
    param: &OpenRequest,
    bot_token: &str,
) -> Result<OpenResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.open");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::channels::channel::{Purpose, Topic};
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = OpenRequest {
            channel: Some("G1234567890".to_string()),
            prevent_creation: Some(true),
            return_im: Some(true),
            users: Some("W1234567890,U2345678901,U3456789012".to_string()),
        };
        let json = r##"{
  "channel": "G1234567890",
  "prevent_creation": true,
  "return_im": true,
  "users": "W1234567890,U2345678901,U3456789012"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<OpenRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = OpenResponse {
            ok: true,
            channel: Some(Channel {
                id: Some("C0EAQDV4Z".to_string()),
                name: Some("endeavor".to_string()),
                is_channel: Some(true),
                created: Some(1504554479),
                creator: Some("U0123456".to_string()),
                is_archived: Some(false),
                is_general: Some(false),
                name_normalized: Some("endeavor".to_string()),
                is_shared: Some(false),
                is_org_shared: Some(false),
                is_member: Some(false),
                is_private: Some(false),
                is_mpim: Some(false),
                last_read: Some("0000000000.000000".to_string()),
                unread_count: Some(0),
                unread_count_display: Some(0),
                topic: Some(Topic {
                    value: Some("".to_string()),
                    creator: Some("".to_string()),
                    last_set: Some(0),
                }),
                purpose: Some(Purpose {
                    value: Some("".to_string()),
                    creator: Some("".to_string()),
                    last_set: Some(0),
                }),
                previous_names: Some(vec![]),
                priority: Some(0),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "channel": {
    "id": "C0EAQDV4Z",
    "name": "endeavor",
    "is_channel": true,
    "created": 1504554479,
    "creator": "U0123456",
    "is_archived": false,
    "is_general": false,
    "name_normalized": "endeavor",
    "is_shared": false,
    "is_org_shared": false,
    "is_member": false,
    "is_private": false,
    "is_mpim": false,
    "last_read": "0000000000.000000",
    "unread_count": 0,
    "unread_count_display": 0,
    "topic": {
      "value": "",
      "creator": "",
      "last_set": 0
    },
    "purpose": {
      "value": "",
      "creator": "",
      "last_set": 0
    },
    "previous_names": [],
    "priority": 0
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<OpenResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_open() {
        let param = OpenRequest {
            channel: Some("G1234567890".to_string()),
            prevent_creation: Some(true),
            return_im: Some(true),
            users: Some("W1234567890,U2345678901,U3456789012".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "channel": {
    "id": "G1234567890",
    "name": "endeavor",
    "is_channel": true,
    "created": 1504554479,
    "creator": "U0123456",
    "is_archived": false,
    "is_general": false,
    "name_normalized": "endeavor",
    "is_shared": false,
    "is_org_shared": false,
    "is_member": false,
    "is_private": false,
    "is_mpim": false,
    "last_read": "0000000000.000000",
    "unread_count": 0,
    "unread_count_display": 0,
    "topic": {
      "value": "",
      "creator": "",
      "last_set": 0
    },
    "purpose": {
      "value": "",
      "creator": "",
      "last_set": 0
    },
    "previous_names": [],
    "priority": 0
  }
}"##
            .to_string())
        });

        let response = open(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = OpenResponse {
            ok: true,
            channel: Some(Channel {
                id: Some("G1234567890".to_string()),
                name: Some("endeavor".to_string()),
                is_channel: Some(true),
                created: Some(1504554479),
                creator: Some("U0123456".to_string()),
                is_archived: Some(false),
                is_general: Some(false),
                name_normalized: Some("endeavor".to_string()),
                is_shared: Some(false),
                is_org_shared: Some(false),
                is_member: Some(false),
                is_private: Some(false),
                is_mpim: Some(false),
                last_read: Some("0000000000.000000".to_string()),
                unread_count: Some(0),
                unread_count_display: Some(0),
                topic: Some(Topic {
                    value: Some("".to_string()),
                    creator: Some("".to_string()),
                    last_set: Some(0),
                }),
                purpose: Some(Purpose {
                    value: Some("".to_string()),
                    creator: Some("".to_string()),
                    last_set: Some(0),
                }),
                previous_names: Some(vec![]),
                priority: Some(0),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
