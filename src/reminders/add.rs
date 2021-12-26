use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::reminders::recurrence::Recurrence;
use crate::reminders::reminder::Reminder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct AddRequest {
    pub text: String,
    pub time: String,
    pub recurrence: Option<Recurrence>,
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct AddResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub reminder: Option<Reminder>,
}

pub async fn add<T>(client: &T, param: &AddRequest, bot_token: &str) -> Result<AddResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reminders.add");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = AddRequest {
            text: "eat a banana".to_string(),
            time: "1602288000".to_string(),
            recurrence: Some(Recurrence {
                frequency: Some("weekly".to_string()),
                weekdays: Some(vec![
                    "monday".to_string(),
                    "wednesday".to_string(),
                    "friday".to_string(),
                ]),
            }),
            team_id: Some("T1234567890".to_string()),
            user: Some("U18888888".to_string()),
        };
        let json = r##"{
  "text": "eat a banana",
  "time": "1602288000",
  "recurrence": {
    "frequency": "weekly",
    "weekdays": [
      "monday",
      "wednesday",
      "friday"
    ]
  },
  "team_id": "T1234567890",
  "user": "U18888888"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<AddRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = AddResponse {
            ok: true,
            reminder: Some(Reminder {
                id: Some("Rm12345678".to_string()),
                creator: Some("U18888888".to_string()),
                user: Some("U18888888".to_string()),
                text: Some("eat a banana".to_string()),
                recurring: Some(false),
                time: Some(1602288000),
                complete_ts: Some(0),
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "reminder": {
    "id": "Rm12345678",
    "creator": "U18888888",
    "user": "U18888888",
    "text": "eat a banana",
    "recurring": false,
    "time": 1602288000,
    "complete_ts": 0
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<AddResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_add() {
        let param = AddRequest {
            text: "eat a banana".to_string(),
            time: "1602288000".to_string(),
            recurrence: Some(Recurrence {
                frequency: Some("weekly".to_string()),
                weekdays: Some(vec![
                    "monday".to_string(),
                    "wednesday".to_string(),
                    "friday".to_string(),
                ]),
            }),
            team_id: Some("T1234567890".to_string()),
            user: Some("U18888888".to_string()),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "reminder": {
    "id": "Rm12345678",
    "creator": "U18888888",
    "user": "U18888888",
    "text": "eat a banana",
    "recurring": false,
    "time": 1602288000,
    "complete_ts": 0
  }
}"##
            .to_string())
        });

        let response = add(&mock, &param, &"test_token".to_string()).await.unwrap();
        let expect = AddResponse {
            ok: true,
            reminder: Some(Reminder {
                id: Some("Rm12345678".to_string()),
                creator: Some("U18888888".to_string()),
                user: Some("U18888888".to_string()),
                text: Some("eat a banana".to_string()),
                recurring: Some(false),
                time: Some(1602288000),
                complete_ts: Some(0),
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
