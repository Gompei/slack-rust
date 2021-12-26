use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::usergroups::usergroup::Usergroup;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct EnableRequest {
    pub usergroup: String,
    pub include_count: Option<bool>,
    pub team_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct EnableResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub usergroup: Option<Usergroup>,
}

pub async fn enable<T>(
    client: &T,
    param: &EnableRequest,
    bot_token: &str,
) -> Result<EnableResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("usergroups.enable");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<EnableResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;
    use crate::usergroups::usergroup::Pref;

    #[test]
    fn convert_request() {
        let request = EnableRequest {
            usergroup: "S0604QSJC".to_string(),
            include_count: Some(true),
            team_id: Some("T1234567890".to_string()),
        };
        let json = r##"{
  "usergroup": "S0604QSJC",
  "include_count": true,
  "team_id": "T1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<EnableRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = EnableResponse {
            ok: true,
            usergroup: Some(Usergroup {
                id: Some("S0615G0KT".to_string()),
                team_id: Some("T060RNRCH".to_string()),
                is_usergroup: Some(true),
                name: Some("Marketing Team".to_string()),
                description: Some("Marketing gurus, PR experts and product advocates.".to_string()),
                handle: Some("marketing-team".to_string()),
                is_external: Some(false),
                date_create: Some(1446746793),
                date_update: Some(1446746793),
                date_delete: Some(0),
                auto_type: Some("".to_string()),
                created_by: Some("U060RNRCZ".to_string()),
                updated_by: Some("U060RNRCZ".to_string()),
                deleted_by: Some("".to_string()),
                prefs: Some(Pref {
                    channels: Some(vec![]),
                    groups: Some(vec![]),
                }),
                user_count: Some("0".to_string()),
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "usergroup": {
    "id": "S0615G0KT",
    "team_id": "T060RNRCH",
    "is_usergroup": true,
    "name": "Marketing Team",
    "description": "Marketing gurus, PR experts and product advocates.",
    "handle": "marketing-team",
    "is_external": false,
    "date_create": 1446746793,
    "date_update": 1446746793,
    "date_delete": 0,
    "auto_type": "",
    "created_by": "U060RNRCZ",
    "updated_by": "U060RNRCZ",
    "deleted_by": "",
    "prefs": {
      "channels": [],
      "groups": []
    },
    "user_count": "0"
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<EnableResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_enable() {
        let param = EnableRequest {
            usergroup: "S0604QSJC".to_string(),
            team_id: Some("T1234567890".to_string()),
            include_count: Some(true),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "usergroup": {
    "id": "S0604QSJC",
    "team_id": "T1234567890",
    "is_usergroup": true,
    "name": "Marketing Team",
    "description": "Marketing gurus, PR experts and product advocates.",
    "handle": "marketing-team",
    "is_external": false,
    "date_create": 1446746793,
    "date_update": 1446746793,
    "date_delete": 0,
    "auto_type": "",
    "created_by": "U060RNRCZ",
    "updated_by": "U060RNRCZ",
    "deleted_by": "",
    "prefs": {
      "channels": [],
      "groups": []
    },
    "user_count": "0"
  }
}"##
            .to_string())
        });

        let response = enable(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = EnableResponse {
            ok: true,
            usergroup: Some(Usergroup {
                id: Some("S0604QSJC".to_string()),
                team_id: Some("T1234567890".to_string()),
                is_usergroup: Some(true),
                name: Some("Marketing Team".to_string()),
                description: Some("Marketing gurus, PR experts and product advocates.".to_string()),
                handle: Some("marketing-team".to_string()),
                is_external: Some(false),
                date_create: Some(1446746793),
                date_update: Some(1446746793),
                date_delete: Some(0),
                auto_type: Some("".to_string()),
                created_by: Some("U060RNRCZ".to_string()),
                updated_by: Some("U060RNRCZ".to_string()),
                deleted_by: Some("".to_string()),
                prefs: Some(Pref {
                    channels: Some(vec![]),
                    groups: Some(vec![]),
                }),
                user_count: Some("0".to_string()),
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
