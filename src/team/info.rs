use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::team::teams::Team;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoRequest {
    pub team: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub team: Option<Team>,
}

pub async fn info<T>(
    client: &T,
    param: &InfoRequest,
    bot_token: &str,
) -> Result<InfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.info");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;
    use crate::team::teams::Icon;

    #[test]
    fn convert_request() {
        let request = InfoRequest {
            team: Some("T1234567890".to_string()),
        };
        let json = r##"{
  "team": "T1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<InfoRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = InfoResponse {
            ok: true,
            team: Some(Team {
                id: Some("T12345".to_string()),
                name: Some("My Team".to_string()),
                domain: Some("example".to_string()),
                email_domain: Some("example.com".to_string()),
                icon: Some(Icon {
                    image_34: Some("https://...".to_string()),
                    image_44: Some("https://...".to_string()),
                    image_68: Some("https://...".to_string()),
                    image_88: Some("https://...".to_string()),
                    image_102: Some("https://...".to_string()),
                    image_132: Some("https://...".to_string()),
                    image_default: Some(true),
                }),
                enterprise_id: Some("E1234A12AB".to_string()),
                enterprise_name: Some("Umbrella Corporation".to_string()),
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "team": {
    "id": "T12345",
    "name": "My Team",
    "domain": "example",
    "email_domain": "example.com",
    "icon": {
      "image_34": "https://...",
      "image_44": "https://...",
      "image_68": "https://...",
      "image_88": "https://...",
      "image_102": "https://...",
      "image_132": "https://...",
      "image_default": true
    },
    "enterprise_id": "E1234A12AB",
    "enterprise_name": "Umbrella Corporation"
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<InfoResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_info() {
        let param = InfoRequest {
            team: Some("T12345".to_string()),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "team": {
    "id": "T12345",
    "name": "My Team",
    "domain": "example",
    "email_domain": "example.com",
    "icon": {
      "image_34": "https://...",
      "image_44": "https://...",
      "image_68": "https://...",
      "image_88": "https://...",
      "image_102": "https://...",
      "image_132": "https://...",
      "image_default": true
    },
    "enterprise_id": "E1234A12AB",
    "enterprise_name": "Umbrella Corporation"
  }
}"##
            .to_string())
        });

        let response = info(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();

        let expect = InfoResponse {
            ok: true,
            team: Some(Team {
                id: Some("T12345".to_string()),
                name: Some("My Team".to_string()),
                domain: Some("example".to_string()),
                email_domain: Some("example.com".to_string()),
                icon: Some(Icon {
                    image_34: Some("https://...".to_string()),
                    image_44: Some("https://...".to_string()),
                    image_68: Some("https://...".to_string()),
                    image_88: Some("https://...".to_string()),
                    image_102: Some("https://...".to_string()),
                    image_132: Some("https://...".to_string()),
                    image_default: Some(true),
                }),
                enterprise_id: Some("E1234A12AB".to_string()),
                enterprise_name: Some("Umbrella Corporation".to_string()),
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
