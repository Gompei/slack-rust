use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::team::teams::Team;
use crate::users::user::User;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct IdentityResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub user: Option<User>,
    pub team: Option<Team>,
}

pub async fn identity<T>(client: &T, bot_token: &str) -> Result<IdentityResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.identity");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<IdentityResponse>(&result).map_err(Error::SerdeJsonError)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_response() {
        let response = IdentityResponse {
            ok: true,
            user: Some(User {
                id: Some("Sonny Whether".to_string()),
                name: Some("U0G9QF9C6".to_string()),
                ..Default::default()
            }),
            team: Some(Team {
                id: Some("T0G9PQBBK".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "user": {
    "id": "Sonny Whether",
    "name": "U0G9QF9C6"
  },
  "team": {
    "id": "T0G9PQBBK"
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<IdentityResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_identity() {
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post().returning(|_, _| {
            Ok(r##"{
  "ok": true,
  "user": {
    "id": "Sonny Whether",
    "name": "U0G9QF9C6"
  },
  "team": {
    "id": "T0G9PQBBK"
  }
}"##
            .to_string())
        });

        let response = identity(&mock, &"test_token".to_string()).await.unwrap();
        let expect = IdentityResponse {
            ok: true,
            user: Some(User {
                id: Some("Sonny Whether".to_string()),
                name: Some("U0G9QF9C6".to_string()),
                ..Default::default()
            }),
            team: Some(Team {
                id: Some("T0G9PQBBK".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
