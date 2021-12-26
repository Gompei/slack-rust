use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct AcceptSharedInviteRequest {
    pub channel_name: String,
    pub channel_id: Option<String>,
    pub free_trial_accepted: Option<bool>,
    pub invite_id: Option<String>,
    pub is_private: Option<bool>,
    pub team_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct AcceptSharedInviteResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub implicit_approval: Option<bool>,
    pub channel_id: Option<String>,
    pub invite_id: Option<String>,
}

pub async fn accept_shared_invite<T>(
    client: &T,
    param: &AcceptSharedInviteRequest,
    bot_token: &str,
) -> Result<AcceptSharedInviteResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.acceptSharedInvite");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<AcceptSharedInviteResponse>(&result)
                .map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = AcceptSharedInviteRequest {
            channel_name: "puppies-r-us".to_string(),
            channel_id: Some("xxxxxxxxxxx".to_string()),
            free_trial_accepted: Some(true),
            invite_id: Some("xxxxxxxxxxx".to_string()),
            is_private: Some(true),
            team_id: Some("T1234567890".to_string()),
        };
        let json = r##"{
  "channel_name": "puppies-r-us",
  "channel_id": "xxxxxxxxxxx",
  "free_trial_accepted": true,
  "invite_id": "xxxxxxxxxxx",
  "is_private": true,
  "team_id": "T1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<AcceptSharedInviteRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = AcceptSharedInviteResponse {
            ok: true,
            implicit_approval: Some(true),
            channel_id: Some("C0001111".to_string()),
            invite_id: Some("I00043221".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "implicit_approval": true,
  "channel_id": "C0001111",
  "invite_id": "I00043221"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<AcceptSharedInviteResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_accept_shared_invite() {
        let param = AcceptSharedInviteRequest {
            channel_name: "puppies-r-us".to_string(),
            channel_id: Some("xxxxxxxxxxx".to_string()),
            free_trial_accepted: Some(true),
            invite_id: Some("xxxxxxxxxxx".to_string()),
            is_private: Some(true),
            team_id: Some("T1234567890".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "implicit_approval": true,
  "channel_id": "xxxxxxxxxxx",
  "invite_id": "xxxxxxxxxxx"
}"##
            .to_string())
        });

        let response = accept_shared_invite(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = AcceptSharedInviteResponse {
            ok: true,
            implicit_approval: Some(true),
            channel_id: Some("xxxxxxxxxxx".to_string()),
            invite_id: Some("xxxxxxxxxxx".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
