use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::users::user::User;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ListRequest {
    pub cursor: Option<String>,
    pub include_locale: Option<bool>,
    pub limit: Option<i32>,
    pub team_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub members: Option<Vec<User>>,
    pub cache_ts: Option<i32>,
}

pub async fn list<T>(
    client: &T,
    param: &ListRequest,
    bot_token: &str,
) -> Result<ListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    
    
    
    
    
    use crate::http_client::MockSlackWebAPIClient;
    
    use crate::users::user::UserProfile;

    #[test]
    fn convert_request() {
        let request = ListRequest {
            cursor: Some("test".to_string()),
            include_locale: Some(true),
            limit: Some(1),
            team_id: Some("test".to_string()),
        };
        let json = r##"{
  "cursor": "test",
  "include_locale": true,
  "limit": 1,
  "team_id": "test"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ListRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = ListResponse {
            ok: true,
            members: Some(vec![User {
                id: Some("USLAOCKO".to_string()),
                team_id: Some("T02H783QNL9".to_string()),
                name: Some("slackbot".to_string()),
                deleted: Some(false),
                color: Some("757575".to_string()),
                real_name: Some("Slackbot".to_string()),
                tz: Some("America/Los_Angeles".to_string()),
                tz_label: Some("Pacific Standard Time".to_string()),
                tz_offset: Some(-28800),
                profile: Some(UserProfile {
                    title: Some("test".to_string()),
                    phone: Some("test".to_string()),
                    skype: Some("test".to_string()),
                    real_name: Some("Slackbot".to_string()),
                    real_name_normalized: Some("Slackbot".to_string()),
                    display_name: Some("Slackbot".to_string()),
                    display_name_normalized: Some("Slackbot".to_string()),
                    status_text: Some("test".to_string()),
                    status_emoji: Some("test".to_string()),
                    status_expiration: Some(0),
                    always_active: Some(true),
                    first_name: Some("slackbot".to_string()),
                    last_name: Some("test".to_string()),
                    image_24: Some("https://example.com/img/slackbot_24.png".to_string()),
                    image_32: Some("https://example.com/img/slackbot_32.png".to_string()),
                    image_48: Some("https://example.com/img/slackbot_48.png".to_string()),
                    image_72: Some("https://example.com//img/slackbot_72.png".to_string()),
                    image_192: Some("https://example.com/img/avatar-slackbot.png".to_string()),
                    image_512: Some("https://example.com/img/slackbot_512.png".to_string()),
                    status_text_canonical: Some("".to_string()),
                    team: Some("T02H7RHQNL9".to_string()),
                    ..Default::default()
                }),
                is_admin: Some(false),
                is_owner: Some(false),
                is_primary_owner: Some(false),
                is_restricted: Some(false),
                is_ultra_restricted: Some(false),
                is_bot: Some(false),
                is_app_user: Some(false),
                updated: Some(0),
                is_email_confirmed: Some(false),
                who_can_share_contact_card: Some("EVERYONE".to_string()),
                ..Default::default()
            }]),
            cache_ts: Some(1111),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "members": [
    {
      "id": "USLAOCKO",
      "team_id": "T02H783QNL9",
      "name": "slackbot",
      "deleted": false,
      "color": "757575",
      "real_name": "Slackbot",
      "tz": "America/Los_Angeles",
      "tz_label": "Pacific Standard Time",
      "tz_offset": -28800,
      "profile": {
        "first_name": "slackbot",
        "last_name": "test",
        "real_name": "Slackbot",
        "real_name_normalized": "Slackbot",
        "display_name": "Slackbot",
        "display_name_normalized": "Slackbot",
        "skype": "test",
        "phone": "test",
        "image_24": "https://example.com/img/slackbot_24.png",
        "image_32": "https://example.com/img/slackbot_32.png",
        "image_48": "https://example.com/img/slackbot_48.png",
        "image_72": "https://example.com//img/slackbot_72.png",
        "image_192": "https://example.com/img/avatar-slackbot.png",
        "image_512": "https://example.com/img/slackbot_512.png",
        "title": "test",
        "status_text": "test",
        "status_emoji": "test",
        "status_expiration": 0,
        "team": "T02H7RHQNL9",
        "always_active": true,
        "status_text_canonical": ""
      },
      "is_bot": false,
      "is_admin": false,
      "is_owner": false,
      "is_primary_owner": false,
      "is_restricted": false,
      "is_ultra_restricted": false,
      "is_app_user": false,
      "updated": 0,
      "is_email_confirmed": false,
      "who_can_share_contact_card": "EVERYONE"
    }
  ],
  "cache_ts": 1111
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ListResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_list() {
        let param = ListRequest {
            team_id: Some("test_list".to_string()),
            ..Default::default()
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "members": [
    {
      "id": "USLAOCKO",
      "team_id": "test_list",
      "name": "slackbot",
      "deleted": false,
      "color": "757575",
      "real_name": "Slackbot",
      "tz": "America/Los_Angeles",
      "tz_label": "Pacific Standard Time",
      "tz_offset": -28800,
      "profile": {
        "first_name": "slackbot",
        "last_name": "test",
        "real_name": "Slackbot",
        "real_name_normalized": "Slackbot",
        "display_name": "Slackbot",
        "display_name_normalized": "Slackbot",
        "skype": "test",
        "phone": "test",
        "image_24": "https://example.com/img/slackbot_24.png",
        "image_32": "https://example.com/img/slackbot_32.png",
        "image_48": "https://example.com/img/slackbot_48.png",
        "image_72": "https://example.com//img/slackbot_72.png",
        "image_192": "https://example.com/img/avatar-slackbot.png",
        "image_512": "https://example.com/img/slackbot_512.png",
        "title": "test",
        "status_text": "test",
        "status_emoji": "test",
        "status_expiration": 0,
        "team": "T02H7RHQNL9",
        "always_active": true,
        "status_text_canonical": ""
      },
      "is_bot": false,
      "is_admin": false,
      "is_owner": false,
      "is_primary_owner": false,
      "is_restricted": false,
      "is_ultra_restricted": false,
      "is_app_user": false,
      "updated": 0,
      "is_email_confirmed": false,
      "who_can_share_contact_card": "EVERYONE"
    }
  ],
  "cache_ts": 1111
}"##
            .to_string())
        });

        let response = list(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = ListResponse {
            ok: true,
            members: Some(vec![User {
                id: Some("USLAOCKO".to_string()),
                team_id: Some("test_list".to_string()),
                name: Some("slackbot".to_string()),
                deleted: Some(false),
                color: Some("757575".to_string()),
                real_name: Some("Slackbot".to_string()),
                tz: Some("America/Los_Angeles".to_string()),
                tz_label: Some("Pacific Standard Time".to_string()),
                tz_offset: Some(-28800),
                profile: Some(UserProfile {
                    title: Some("test".to_string()),
                    phone: Some("test".to_string()),
                    skype: Some("test".to_string()),
                    real_name: Some("Slackbot".to_string()),
                    real_name_normalized: Some("Slackbot".to_string()),
                    display_name: Some("Slackbot".to_string()),
                    display_name_normalized: Some("Slackbot".to_string()),
                    status_text: Some("test".to_string()),
                    status_emoji: Some("test".to_string()),
                    status_expiration: Some(0),
                    always_active: Some(true),
                    first_name: Some("slackbot".to_string()),
                    last_name: Some("test".to_string()),
                    image_24: Some("https://example.com/img/slackbot_24.png".to_string()),
                    image_32: Some("https://example.com/img/slackbot_32.png".to_string()),
                    image_48: Some("https://example.com/img/slackbot_48.png".to_string()),
                    image_72: Some("https://example.com//img/slackbot_72.png".to_string()),
                    image_192: Some("https://example.com/img/avatar-slackbot.png".to_string()),
                    image_512: Some("https://example.com/img/slackbot_512.png".to_string()),
                    status_text_canonical: Some("".to_string()),
                    team: Some("T02H7RHQNL9".to_string()),
                    ..Default::default()
                }),
                is_admin: Some(false),
                is_owner: Some(false),
                is_primary_owner: Some(false),
                is_restricted: Some(false),
                is_ultra_restricted: Some(false),
                is_bot: Some(false),
                is_app_user: Some(false),
                updated: Some(0),
                is_email_confirmed: Some(false),
                who_can_share_contact_card: Some("EVERYONE".to_string()),
                ..Default::default()
            }]),
            cache_ts: Some(1111),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
