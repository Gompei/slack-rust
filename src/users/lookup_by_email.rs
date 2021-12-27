use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::users::user::User;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct LookupByEmailRequest {
    pub email: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct LookupByEmailResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub user: Option<User>,
}

pub async fn lookup_by_email<T>(
    client: &T,
    param: &LookupByEmailRequest,
    bot_token: &str,
) -> Result<LookupByEmailResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.lookupByEmail");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<LookupByEmailResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;
    use crate::users::user::UserProfile;

    #[test]
    fn convert_request() {
        let request = LookupByEmailRequest {
            email: "spengler@ghostbusters.example.com".to_string(),
        };
        let json = r##"{
  "email": "spengler@ghostbusters.example.com"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<LookupByEmailRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = LookupByEmailResponse {
            ok: true,
            user: Some(User {
                id: Some("Sonny Whether".to_string()),
                name: Some("U0G9QF9C6".to_string()),
                profile: Some(UserProfile {
                    email: Some("spengler@ghostbusters.example.com".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "user": {
    "id": "Sonny Whether",
    "name": "U0G9QF9C6",
    "profile": {
      "email": "spengler@ghostbusters.example.com"
    }
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<LookupByEmailResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_lookup_by_email() {
        let param = LookupByEmailRequest {
            email: "spengler@ghostbusters.example.com".to_string(),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "user": {
    "id": "Sonny Whether",
    "name": "U0G9QF9C6",
    "profile": {
      "email": "spengler@ghostbusters.example.com"
    }
  }
}"##
            .to_string())
        });

        let response = lookup_by_email(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = LookupByEmailResponse {
            ok: true,
            user: Some(User {
                id: Some("Sonny Whether".to_string()),
                name: Some("U0G9QF9C6".to_string()),
                profile: Some(UserProfile {
                    email: Some("spengler@ghostbusters.example.com".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
