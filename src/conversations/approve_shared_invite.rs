//! Approves an invitation to a Slack Connect channel.

use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ApproveSharedInviteRequest {
    pub invite_id: String,
    pub target_team: Option<String>,
}

/// Approves an invitation to a Slack Connect channel.  
/// See: <https://api.slack.com/methods/conversations.approveSharedInvite>
pub async fn approve_shared_invite<T>(
    client: &T,
    param: &ApproveSharedInviteRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.approveSharedInvite");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = ApproveSharedInviteRequest {
            invite_id: "xxxxxxxxxxx".to_string(),
            target_team: Some("xxxxxxxxxxx".to_string()),
        };
        let json = r##"{
  "invite_id": "xxxxxxxxxxx",
  "target_team": "xxxxxxxxxxx"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ApproveSharedInviteRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[async_std::test]
    async fn test_approve_shared_invite() {
        let param = ApproveSharedInviteRequest {
            invite_id: "xxxxxxxxxxx".to_string(),
            target_team: Some("xxxxxxxxxxx".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true
}"##
            .to_string())
        });

        let response = approve_shared_invite(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
