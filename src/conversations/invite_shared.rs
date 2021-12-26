use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InviteSharedRequest {
    pub channel: String,
    pub emails: Option<Vec<String>>,
    pub external_limited: Option<bool>,
    pub user_ids: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InviteSharedResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub invite_id: Option<String>,
    pub conf_code: Option<String>,
    pub url: Option<String>,
    pub is_legacy_shared_channel: Option<bool>,
}

pub async fn invite_shared<T>(
    client: &T,
    param: &InviteSharedRequest,
    bot_token: &str,
) -> Result<InviteSharedResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.inviteShared");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InviteSharedResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = InviteSharedRequest {
            channel: "C1234567890".to_string(),
            emails: Some(vec!["xxxxxxxxxxx".to_string(), "xxxxxxxxxxx".to_string()]),
            external_limited: Some(true),
            user_ids: Some(vec!["xxxxxxxxxxx".to_string(), "xxxxxxxxxxx".to_string()]),
        };
        let json = r##"{
  "channel": "C1234567890",
  "emails": [
    "xxxxxxxxxxx",
    "xxxxxxxxxxx"
  ],
  "external_limited": true,
  "user_ids": [
    "xxxxxxxxxxx",
    "xxxxxxxxxxx"
  ]
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<InviteSharedRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = InviteSharedResponse {
            ok: true,
            invite_id: Some("I011K7UESHG".to_string()),
            conf_code: Some("dYhjpyzi8GHzRSol1RoLKurD".to_string()),
            url: Some("https://join.slack.com/share/9hhxk2455rosq89X1ng".to_string()),
            is_legacy_shared_channel: Some(false),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "invite_id": "I011K7UESHG",
  "conf_code": "dYhjpyzi8GHzRSol1RoLKurD",
  "url": "https://join.slack.com/share/9hhxk2455rosq89X1ng",
  "is_legacy_shared_channel": false
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<InviteSharedResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_invite_shared() {
        let param = InviteSharedRequest {
            channel: "C1234567890".to_string(),
            emails: Some(vec!["xxxxxxxxxxx".to_string(), "xxxxxxxxxxx".to_string()]),
            external_limited: Some(true),
            user_ids: Some(vec!["xxxxxxxxxxx".to_string(), "xxxxxxxxxxx".to_string()]),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "invite_id": "I011K7UESHG",
  "conf_code": "dYhjpyzi8GHzRSol1RoLKurD",
  "url": "https://join.slack.com/share/9hhxk2455rosq89X1ng",
  "is_legacy_shared_channel": false
}"##
            .to_string())
        });

        let response = invite_shared(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = InviteSharedResponse {
            ok: true,
            invite_id: Some("I011K7UESHG".to_string()),
            conf_code: Some("dYhjpyzi8GHzRSol1RoLKurD".to_string()),
            url: Some("https://join.slack.com/share/9hhxk2455rosq89X1ng".to_string()),
            is_legacy_shared_channel: Some(false),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
