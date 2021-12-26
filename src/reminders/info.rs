use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::reminders::reminder::Reminder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoRequest {
    pub reminder: String,
    pub team_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub reminder: Option<Reminder>,
}

pub async fn info<T>(
    client: &T,
    param: &InfoRequest,
    bot_token: &str,
) -> Result<InfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reminders.info");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = InfoRequest {
            reminder: "Rm12345678".to_string(),
            team_id: Some("T1234567890".to_string()),
        };
        let json = r##"{
  "reminder": "Rm12345678",
  "team_id": "T1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<InfoRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = InfoResponse {
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

        let s = serde_json::from_str::<InfoResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_info() {
        let param = InfoRequest {
            reminder: "Rm12345678".to_string(),
            team_id: Some("T1234567890".to_string()),
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

        let response = info(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = InfoResponse {
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
