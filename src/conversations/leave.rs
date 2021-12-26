use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct LeaveRequest {
    pub channel: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct LeaveResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub not_in_channel: Option<bool>,
}

pub async fn leave<T>(
    client: &T,
    param: &LeaveRequest,
    bot_token: &str,
) -> Result<LeaveResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.leave");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<LeaveResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = LeaveRequest {
            channel: "C1234567890".to_string(),
        };
        let json = r##"{
  "channel": "C1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<LeaveRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = LeaveResponse {
            ok: true,
            not_in_channel: Some(true),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "not_in_channel": true
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<LeaveResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_leave() {
        let param = LeaveRequest {
            channel: "C1234567890".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "not_in_channel": true
}"##
            .to_string())
        });

        let response = leave(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = LeaveResponse {
            ok: true,
            not_in_channel: Some(true),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
