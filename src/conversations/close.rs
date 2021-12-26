use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct CloseRequest {
    pub channel: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct CloseResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub no_op: Option<bool>,
    pub already_closed: Option<bool>,
}

pub async fn close<T>(
    client: &T,
    param: &CloseRequest,
    bot_token: &str,
) -> Result<CloseResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.close");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<CloseResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = CloseRequest {
            channel: "G1234567890".to_string(),
        };
        let json = r##"{
  "channel": "G1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<CloseRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = CloseResponse {
            ok: true,
            no_op: Some(true),
            already_closed: Some(true),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "no_op": true,
  "already_closed": true
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<CloseResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_close() {
        let param = CloseRequest {
            channel: "G1234567890".to_string(),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "no_op": true,
  "already_closed": true
}"##
            .to_string())
        });

        let response = close(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = CloseResponse {
            ok: true,
            no_op: Some(true),
            already_closed: Some(true),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
