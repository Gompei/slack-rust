use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::items::item::Item;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ListRequest {
    pub count: Option<i32>,
    pub cursor: Option<String>,
    pub full: Option<bool>,
    pub limit: Option<i32>,
    pub page: Option<i32>,
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    // TODO
    // https://api.slack.com/methods/reactions.list#examples
    pub items: Option<Vec<Item>>,
}

pub async fn list<T>(
    client: &T,
    param: &ListRequest,
    bot_token: &str,
) -> Result<ListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reactions.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::chat::message::Message;
    use crate::http_client::MockSlackWebAPIClient;
    use crate::reactions::reaction::Reaction;

    #[test]
    fn convert_request() {
        let request = ListRequest {
            count: Some(20),
            full: Some(true),
            limit: Some(20),
            page: Some(2),
            team_id: Some("T1234567890".to_string()),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            user: Some("W1234567890".to_string()),
        };
        let json = r##"{
  "count": 20,
  "cursor": "dXNlcjpVMDYxTkZUVDI=",
  "full": true,
  "limit": 20,
  "page": 2,
  "team_id": "T1234567890",
  "user": "W1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ListRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = ListResponse {
            ok: true,
            items: Some(vec![Item {
                type_filed: Some("message".to_string()),
                channel: Some("C3UKJTQAC".to_string()),
                message: Some(Message {
                    bot_id: Some("B4VLRLMKJ".to_string()),
                    text: Some("Hello from Python! :tada:".to_string()),
                    username: Some("Shipit Notifications".to_string()),
                    ts: Some("1507849573.000090".to_string()),
                    subtype: Some("bot_message".to_string()),
                    reactions: Some(vec![Reaction {
                        count: Some(1),
                        name: Some("robot_face".to_string()),
                        users: Some(vec!["U2U85N1RV".to_string()]),
                    }]),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "items": [
    {
      "type": "message",
      "channel": "C3UKJTQAC",
      "message": {
        "bot_id": "B4VLRLMKJ",
        "text": "Hello from Python! :tada:",
        "username": "Shipit Notifications",
        "ts": "1507849573.000090",
        "subtype": "bot_message",
        "reactions": [
          {
            "count": 1,
            "name": "robot_face",
            "users": [
              "U2U85N1RV"
            ]
          }
        ]
      }
    }
  ]
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ListResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_list() {
        let param = ListRequest {
            count: Some(20),
            full: Some(true),
            limit: Some(20),
            page: Some(2),
            team_id: Some("T1234567890".to_string()),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            user: Some("W1234567890".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "items": [
    {
      "type": "message",
      "channel": "C3UKJTQAC",
      "message": {
        "bot_id": "B4VLRLMKJ",
        "text": "Hello from Python! :tada:",
        "username": "Shipit Notifications",
        "ts": "1507849573.000090",
        "subtype": "bot_message",
        "reactions": [
          {
            "count": 1,
            "name": "robot_face",
            "users": [
              "U2U85N1RV"
            ]
          }
        ]
      }
    }
  ]
}"##
            .to_string())
        });

        let response = list(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = ListResponse {
            ok: true,
            items: Some(vec![Item {
                type_filed: Some("message".to_string()),
                channel: Some("C3UKJTQAC".to_string()),
                message: Some(Message {
                    bot_id: Some("B4VLRLMKJ".to_string()),
                    text: Some("Hello from Python! :tada:".to_string()),
                    username: Some("Shipit Notifications".to_string()),
                    ts: Some("1507849573.000090".to_string()),
                    subtype: Some("bot_message".to_string()),
                    reactions: Some(vec![Reaction {
                        count: Some(1),
                        name: Some("robot_face".to_string()),
                        users: Some(vec!["U2U85N1RV".to_string()]),
                    }]),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
