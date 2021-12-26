use crate::error::Error;
use crate::files::file::File;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetRequest {
    pub channel: Option<String>,
    pub file: Option<String>,
    pub file_comment: Option<String>,
    pub full: Option<bool>,
    pub timestamp: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub file: Option<File>,
    #[serde(rename = "type")]
    pub type_filed: Option<String>,
}

pub async fn get<T>(client: &T, param: &GetRequest, bot_token: &str) -> Result<GetResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reactions.get");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;
    use crate::reactions::reaction::Reaction;

    #[test]
    fn convert_request() {
        let request = GetRequest {
            channel: Some("C0NF841BK".to_string()),
            file: Some("F1234567890".to_string()),
            file_comment: Some("Fc1234567890".to_string()),
            full: Some(true),
            timestamp: Some("1524523204.000192".to_string()),
        };
        let json = r##"{
  "channel": "C0NF841BK",
  "file": "F1234567890",
  "file_comment": "Fc1234567890",
  "full": true,
  "timestamp": "1524523204.000192"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<GetRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = GetResponse {
            ok: true,
            file: Some(File {
                channels: Some(vec!["C2U7V2YA2".to_string()]),
                comments_count: Some(1),
                created: Some(1507850315),
                groups: Some(vec![]),
                id: Some("F7H0D7ZA4".to_string()),
                ims: Some(vec![]),
                name: Some("computer.gif".to_string()),
                reactions: Some(vec![Reaction {
                    count: Some(1),
                    name: Some("stuck_out_tongue_winking_eye".to_string()),
                    users: Some(vec!["U2U85N1RV".to_string()]),
                }]),
                timestamp: Some(1507850315),
                title: Some("computer.gif".to_string()),
                user: Some("U2U85N1RV".to_string()),
            }),
            type_filed: Some("file".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "file": {
    "channels": [
      "C2U7V2YA2"
    ],
    "comments_count": 1,
    "created": 1507850315,
    "groups": [],
    "id": "F7H0D7ZA4",
    "ims": [],
    "name": "computer.gif",
    "reactions": [
      {
        "count": 1,
        "name": "stuck_out_tongue_winking_eye",
        "users": [
          "U2U85N1RV"
        ]
      }
    ],
    "timestamp": 1507850315,
    "title": "computer.gif",
    "user": "U2U85N1RV"
  },
  "type": "file"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<GetResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_get() {
        let param = GetRequest {
            channel: Some("C0NF841BK".to_string()),
            file: Some("F1234567890".to_string()),
            file_comment: Some("Fc1234567890".to_string()),
            full: Some(true),
            timestamp: Some("1524523204.000192".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "file": {
    "channels": [
      "C0NF841BK"
    ],
    "comments_count": 1,
    "created": 1507850315,
    "groups": [],
    "id": "F1234567890",
    "ims": [],
    "name": "computer.gif",
    "reactions": [
      {
        "count": 1,
        "name": "stuck_out_tongue_winking_eye",
        "users": [
          "U2U85N1RV"
        ]
      }
    ],
    "timestamp": 1524523204,
    "title": "computer.gif",
    "user": "U2U85N1RV"
  },
  "type": "file"
}"##
            .to_string())
        });

        let response = get(&mock, &param, &"test_token".to_string()).await.unwrap();
        let expect = GetResponse {
            ok: true,
            file: Some(File {
                channels: Some(vec!["C0NF841BK".to_string()]),
                comments_count: Some(1),
                created: Some(1507850315),
                groups: Some(vec![]),
                id: Some("F1234567890".to_string()),
                ims: Some(vec![]),
                name: Some("computer.gif".to_string()),
                reactions: Some(vec![Reaction {
                    count: Some(1),
                    name: Some("stuck_out_tongue_winking_eye".to_string()),
                    users: Some(vec!["U2U85N1RV".to_string()]),
                }]),
                timestamp: Some(1524523204),
                title: Some("computer.gif".to_string()),
                user: Some("U2U85N1RV".to_string()),
            }),
            type_filed: Some("file".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
