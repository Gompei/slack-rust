//! Retrieve members of a conversation.

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct MembersRequest {
    pub channel: String,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct MembersResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub members: Option<Vec<String>>,
}

/// Retrieve members of a conversation.  
/// See: <https://api.slack.com/methods/conversations.members>
pub async fn members<T>(
    client: &T,
    param: &MembersRequest,
    bot_token: &str,
) -> Result<MembersResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.members");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<MembersResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = MembersRequest {
            channel: "C1234567890".to_string(),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            limit: Some(20),
        };
        let json = r##"{
  "channel": "C1234567890",
  "cursor": "dXNlcjpVMDYxTkZUVDI=",
  "limit": 20
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<MembersRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = MembersResponse {
            ok: true,
            members: Some(vec![
                "U023BECGF".to_string(),
                "U061F7AUR".to_string(),
                "W012A3CDE".to_string(),
            ]),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "members": [
    "U023BECGF",
    "U061F7AUR",
    "W012A3CDE"
  ]
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<MembersResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_members() {
        let param = MembersRequest {
            channel: "C1234567890".to_string(),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            limit: Some(20),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "members": [
    "U023BECGF",
    "U061F7AUR",
    "W012A3CDE"
  ]
}"##
            .to_string())
        });

        let response = members(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = MembersResponse {
            ok: true,
            members: Some(vec![
                "U023BECGF".to_string(),
                "U061F7AUR".to_string(),
                "W012A3CDE".to_string(),
            ]),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
