use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::team::team::Team;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TeamsListRequest {
    pub cursor: Option<String>,
    pub include_icon: Option<bool>,
    pub limit: Option<i32>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TeamsListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub teams: Option<Vec<Team>>,
}

pub async fn teams_list<T>(
    client: &T,
    param: &TeamsListRequest,
    bot_token: &str,
) -> Result<TeamsListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("auth.teams.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<TeamsListResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = TeamsListRequest {
            cursor: Some("5c3e53d5".to_string()),
            include_icon: Some(true),
            limit: Some(1),
        };
        let json = r##"{
  "cursor": "5c3e53d5",
  "include_icon": true,
  "limit": 1
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<TeamsListRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = TeamsListResponse {
            ok: true,
            teams: Some(vec![
                Team {
                    id: Some("T12345678".to_string()),
                    name: Some("Shinichi's workspace".to_string()),
                    ..Default::default()
                },
                Team {
                    id: Some("T12345679".to_string()),
                    name: Some("Migi's workspace".to_string()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "teams": [
    {
      "id": "T12345678",
      "name": "Shinichi's workspace"
    },
    {
      "id": "T12345679",
      "name": "Migi's workspace"
    }
  ]
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<TeamsListResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_teams_list() {
        let param = TeamsListRequest {
            cursor: Some("5c3e53d5".to_string()),
            include_icon: Some(true),
            limit: Some(1),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "teams": [
    {
      "id": "T12345678",
      "name": "Shinichi's workspace"
    },
    {
      "id": "T12345679",
      "name": "Migi's workspace"
    }
  ]
}"##
            .to_string())
        });

        let response = teams_list(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = TeamsListResponse {
            ok: true,
            teams: Some(vec![
                Team {
                    id: Some("T12345678".to_string()),
                    name: Some("Shinichi's workspace".to_string()),
                    ..Default::default()
                },
                Team {
                    id: Some("T12345679".to_string()),
                    name: Some("Migi's workspace".to_string()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
