use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RemoveRequest {
    pub name: String,
    pub channel: Option<String>,
    pub file: Option<String>,
    pub file_comment: Option<String>,
    pub timestamp: Option<String>,
}

pub async fn remove<T>(
    client: &T,
    param: &RemoveRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reactions.remove");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = RemoveRequest {
            name: "thumbsup".to_string(),
            channel: Some("C1234567890".to_string()),
            file: Some("F1234567890".to_string()),
            file_comment: Some("Fc1234567890".to_string()),
            timestamp: Some("1234567890.123456".to_string()),
        };
        let json = r##"{
  "name": "thumbsup",
  "channel": "C1234567890",
  "file": "F1234567890",
  "file_comment": "Fc1234567890",
  "timestamp": "1234567890.123456"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<RemoveRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[async_std::test]
    async fn test_remove() {
        let param = RemoveRequest {
            name: "thumbsup".to_string(),
            channel: Some("C1234567890".to_string()),
            file: Some("F1234567890".to_string()),
            file_comment: Some("Fc1234567890".to_string()),
            timestamp: Some("1234567890.123456".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true
}"##
            .to_string())
        });

        let response = remove(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
