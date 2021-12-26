use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::reminders::reminder::Reminder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ListRequest {
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub reminders: Option<Vec<Reminder>>,
}

pub async fn list<T>(
    client: &T,
    param: &ListRequest,
    bot_token: &str,
) -> Result<ListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reminders.list");
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
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = ListRequest {
            team_id: Some("T1234567890".to_string()),
        };
        let json = r##"{
  "team_id": "T1234567890"
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
            reminders: Some(vec![
                Reminder {
                    id: Some("Rm12345678".to_string()),
                    creator: Some("U18888888".to_string()),
                    user: Some("U18888888".to_string()),
                    text: Some("eat a banana".to_string()),
                    recurring: Some(false),
                    time: Some(1602288000),
                    complete_ts: Some(0),
                },
                Reminder {
                    id: Some("Rm12345678".to_string()),
                    creator: Some("U18888888".to_string()),
                    user: Some("U18888888".to_string()),
                    text: Some("eat a banana".to_string()),
                    recurring: Some(false),
                    time: Some(1602288000),
                    complete_ts: Some(0),
                },
            ]),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "error": null,
  "response_metadata": null,
  "reminders": [
    {
      "id": "Rm12345678",
      "creator": "U18888888",
      "user": "U18888888",
      "text": "eat a banana",
      "recurring": false,
      "time": 1602288000,
      "complete_ts": 0
    },
    {
      "id": "Rm12345678",
      "creator": "U18888888",
      "user": "U18888888",
      "text": "eat a banana",
      "recurring": false,
      "time": 1602288000,
      "complete_ts": 0
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
            team_id: Some("T1234567890".to_string()),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "error": null,
  "response_metadata": null,
  "reminders": [
    {
      "id": "Rm12345678",
      "creator": "U18888888",
      "user": "U18888888",
      "text": "eat a banana",
      "recurring": false,
      "time": 1602288000,
      "complete_ts": 0
    },
    {
      "id": "Rm12345678",
      "creator": "U18888888",
      "user": "U18888888",
      "text": "eat a banana",
      "recurring": false,
      "time": 1602288000,
      "complete_ts": 0
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
            reminders: Some(vec![
                Reminder {
                    id: Some("Rm12345678".to_string()),
                    creator: Some("U18888888".to_string()),
                    user: Some("U18888888".to_string()),
                    text: Some("eat a banana".to_string()),
                    recurring: Some(false),
                    time: Some(1602288000),
                    complete_ts: Some(0),
                },
                Reminder {
                    id: Some("Rm12345678".to_string()),
                    creator: Some("U18888888".to_string()),
                    user: Some("U18888888".to_string()),
                    text: Some("eat a banana".to_string()),
                    recurring: Some(false),
                    time: Some(1602288000),
                    complete_ts: Some(0),
                },
            ]),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
