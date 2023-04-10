use crate::team::teams::Team;
use crate::users::user::User;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use serde_json::Value;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
/// See more at <https://api.slack.com/events/app_requested>
///
/// Note that the `enterprise` field is a bit ambiguous. Slack's official
/// documentation says this value may be null or not null (but does not provide
/// an example). Therefore we use the `serde_json::Value::Null` variant to
/// represent this possibility
pub struct AppRequest {
    pub app: Option<App>,
    pub enterprise: Value,
    pub id: Option<String>,
    pub message: Option<String>,
    pub previous_resolution: Option<PreviousResolution>,
    pub scopes: Option<Vec<Scope>>,
    pub team: Option<Team>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct App {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub app_homepage_url: Option<String>,
    pub app_directory_url: Option<String>,
    pub is_app_directory_approved: Option<bool>,
    pub is_internal: Option<bool>,
    pub additional_info: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PreviousResolution {
    pub status: Option<String>,
    pub scopes: Option<Vec<Scope>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Scope {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_sensitive: Option<bool>,
    pub token_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::event_api::event::EventType;
    use assert_json_diff::*;
    use serde_json::Value;

    #[test]
    fn deserialized_app_requested_event() {
        let json = r##"
        {
          "type": "app_requested",
          "app_request":{
              "id": "1234",
              "app": {
                "id": "A5678",
                "name": "Brent's app",
                "description": "They're good apps, Bront.",
                "help_url": "brontsapp.com",
                "privacy_policy_url": "brontsapp.com",
                "app_homepage_url": "brontsapp.com",
                "app_directory_url": "https://slack.slack.com/apps/A102ARD7Y",
                "is_app_directory_approved": true,
                "is_internal": false,
                "additional_info": "none"
              },
              "previous_resolution": {
                 "status": "approved",
                 "scopes": [
                  {
                    "name": "app_requested",
                    "description": "allows this app to listen for app install requests",
                    "is_sensitive": false,
                    "token_type": "user"
                  }]
              },
              "user":{
                "id": "U1234",
                "name": "Bront",
                "email": "bront@brent.com"
              },
              "team": {
                "id": "T1234",
                "name": "Brant App Team",
                "domain": "brantappteam"
              },
              "enterprise": null,
              "scopes": [
                {
                  "name": "app_requested",
                  "description": "allows this app to listen for app install requests",
                  "is_sensitive": false,
                  "token_type": "user"
                }
              ],
              "message": "none"
          }
        }
        "##;
        let deserialized = serde_json::from_str::<EventType>(&json).unwrap();
        // Make comparison between our deserialized struct and generic serde_json Value to ensure
        // that all expected keys exist
        let expected: Value = serde_json::from_str(&json).unwrap();
        assert_json_eq!(deserialized, expected);

        match deserialized {
            EventType::AppRequested{..} => assert!(true),
            _ => panic!("unrecognized variant"),
        }
    }
}
