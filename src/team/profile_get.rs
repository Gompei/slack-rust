use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::profiles::profile::Profile;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ProfileGetRequest {
    pub visibility: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ProfileGetResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub profile: Option<Profile>,
}

pub async fn profile_get<T>(
    client: &T,
    param: &ProfileGetRequest,
    bot_token: &str,
) -> Result<ProfileGetResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.profile.get");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ProfileGetResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;
    use crate::profiles::profile::Field;

    #[test]
    fn convert_request() {
        let request = ProfileGetRequest {
            visibility: Some("all".to_string()),
        };
        let json = r##"{
  "visibility": "all"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ProfileGetRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = ProfileGetResponse {
            ok: true,
            profile: Some(Profile {
                fields: Some(vec![Field {
                    id: Some("Xf06054AAA".to_string()),
                    ordering: Some(0),
                    label: Some("Phone extension".to_string()),
                    hint: Some("Enter the extension to reach your desk".to_string()),
                    type_filed: Some("text".to_string()),
                    possible_values: Some(vec![]),
                    is_hidden: Some(1),
                    ..Default::default()
                }]),
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "profile": {
    "fields": [
      {
        "id": "Xf06054AAA",
        "ordering": 0,
        "label": "Phone extension",
        "hint": "Enter the extension to reach your desk",
        "type": "text",
        "possible_values": [],
        "is_hidden": 1
      }
    ]
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ProfileGetResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_profile_get() {
        let param = ProfileGetRequest {
            visibility: Some("all".to_string()),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "profile": {
    "fields": [
      {
        "id": "Xf06054AAA",
        "ordering": 0,
        "label": "Phone extension",
        "hint": "Enter the extension to reach your desk",
        "type": "text",
        "possible_values": [],
        "is_hidden": 1
      }
    ]
  }
}"##
            .to_string())
        });

        let response = profile_get(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = ProfileGetResponse {
            ok: true,
            profile: Some(Profile {
                fields: Some(vec![Field {
                    id: Some("Xf06054AAA".to_string()),
                    ordering: Some(0),
                    label: Some("Phone extension".to_string()),
                    hint: Some("Enter the extension to reach your desk".to_string()),
                    type_filed: Some("text".to_string()),
                    possible_values: Some(vec![]),
                    is_hidden: Some(1),
                    ..Default::default()
                }]),
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
