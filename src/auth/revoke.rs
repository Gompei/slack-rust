//! Revokes a token.
//! See: <https://api.slack.com/methods/auth.revoke>

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RevokeRequest {
    pub test: Option<bool>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RevokeResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub revoked: Option<bool>,
}

/// Revokes a token.
/// See: <https://api.slack.com/methods/auth.revoke>
pub async fn revoke<T>(
    client: &T,
    param: &RevokeRequest,
    bot_token: &str,
) -> Result<RevokeResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("auth.revoke");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<RevokeResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = RevokeRequest { test: Some(true) };
        let json = r##"{
  "test": true
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<RevokeRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = RevokeResponse {
            ok: true,
            revoked: Some(true),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "revoked": true
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<RevokeResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_revoke() {
        let param = RevokeRequest { test: Some(false) };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
          "ok": true,
          "revoked": true
        }"##
            .to_string())
        });

        let response = revoke(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = RevokeResponse {
            ok: true,
            revoked: Some(true),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
