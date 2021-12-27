use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::users::user::User;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoRequest {
    pub user: String,
    pub include_locale: Option<bool>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub user: Option<User>,
}

pub async fn info<T>(
    client: &T,
    param: &InfoRequest,
    bot_token: &str,
) -> Result<InfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.info");
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

    #[test]
    fn convert_request() {
        let request = InfoRequest {
            user: "W1234567890".to_string(),
            include_locale: Some(true),
        };
        let json = r##"{
  "user": "W1234567890",
  "include_locale": true
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
            user: Some(User {
                id: Some("Sonny Whether".to_string()),
                name: Some("U0G9QF9C6".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "user": {
    "id": "Sonny Whether",
    "name": "U0G9QF9C6"
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
            user: "W1234567890".to_string(),
            include_locale: Some(true),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "user": {
    "id": "W1234567890",
    "name": "xxxxxxxx"
  },
  "team": {
    "id": "T0G9PQBBK"
  }
}"##
            .to_string())
        });

        let response = info(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = InfoResponse {
            ok: true,
            user: Some(User {
                id: Some("W1234567890".to_string()),
                name: Some("xxxxxxxx".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
