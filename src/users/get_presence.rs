use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetPresenceRequest {
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetPresenceResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub presence: Option<String>,
    pub online: Option<bool>,
    pub auto_away: Option<bool>,
    pub manual_away: Option<bool>,
    pub connection_count: Option<i8>,
    pub last_activity: Option<i32>,
}

pub async fn get_presence<T>(
    client: &T,
    param: &GetPresenceRequest,
    bot_token: &str,
) -> Result<GetPresenceResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.getPresence");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<GetPresenceResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;
    use crate::usergroups::usergroup::Pref;

    #[test]
    fn convert_request() {
        let request = GetPresenceRequest {
            user: Some("xxxxxxxxxx".to_string()),
        };
        let json = r##"{
  "user": "xxxxxxxxxx"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<GetPresenceRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = GetPresenceResponse {
            ok: true,
            presence: Some("active".to_string()),
            online: Some(true),
            auto_away: Some(false),
            manual_away: Some(false),
            connection_count: Some(1),
            last_activity: Some(1419027078),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "presence": "active",
  "online": true,
  "auto_away": false,
  "manual_away": false,
  "connection_count": 1,
  "last_activity": 1419027078
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<GetPresenceResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_get_presence() {
        let param = GetPresenceRequest {
            user: Some("xxxxxxxxxx".to_string()),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "presence": "active",
  "online": true,
  "auto_away": false,
  "manual_away": false,
  "connection_count": 1,
  "last_activity": 1419027078
}"##
            .to_string())
        });

        let response = get_presence(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = GetPresenceResponse {
            ok: true,
            presence: Some("active".to_string()),
            online: Some(true),
            auto_away: Some(false),
            manual_away: Some(false),
            connection_count: Some(1),
            last_activity: Some(1419027078),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
