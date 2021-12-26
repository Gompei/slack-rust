use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoRequest {
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub dnd_enabled: Option<bool>,
    pub next_dnd_start_ts: Option<i32>,
    pub next_dnd_end_ts: Option<i32>,
    pub snooze_enabled: Option<bool>,
    pub snooze_endtime: Option<i32>,
    pub snooze_remaining: Option<i16>,
    pub snooze_is_indefinite: Option<bool>,
}

pub async fn info<T>(
    client: &T,
    param: &InfoRequest,
    bot_token: &str,
) -> Result<InfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("dnd.info");
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
            team_id: Some("T1234567890".to_string()),
            user: Some("U1234".to_string()),
        };
        let json = r##"{
  "team_id": "T1234567890",
  "user": "U1234"
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
            dnd_enabled: Some(true),
            next_dnd_start_ts: Some(1450416600),
            next_dnd_end_ts: Some(1450452600),
            snooze_enabled: Some(true),
            snooze_endtime: Some(1450416600),
            snooze_remaining: Some(1196),
            snooze_is_indefinite: Some(false),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "dnd_enabled": true,
  "next_dnd_start_ts": 1450416600,
  "next_dnd_end_ts": 1450452600,
  "snooze_enabled": true,
  "snooze_endtime": 1450416600,
  "snooze_remaining": 1196,
  "snooze_is_indefinite": false
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<InfoResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_info() {
        let param = InfoRequest {
            team_id: Some("T1234567890".to_string()),
            user: Some("U1234".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "dnd_enabled": true,
  "next_dnd_start_ts": 1450416600,
  "next_dnd_end_ts": 1450452600,
  "snooze_enabled": true,
  "snooze_endtime": 1450416600,
  "snooze_remaining": 1196,
  "snooze_is_indefinite": false
}"##
            .to_string())
        });

        let response = info(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = InfoResponse {
            ok: true,
            dnd_enabled: Some(true),
            next_dnd_start_ts: Some(1450416600),
            next_dnd_end_ts: Some(1450452600),
            snooze_enabled: Some(true),
            snooze_endtime: Some(1450416600),
            snooze_remaining: Some(1196),
            snooze_is_indefinite: Some(false),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
