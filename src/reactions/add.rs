use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct AddRequest {
    pub channel: String,
    pub name: String,
    pub timestamp: String,
}

pub async fn add<T>(
    client: &T,
    param: &AddRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reactions.add");
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
        let request = AddRequest {
            channel: "C1234567890".to_string(),
            name: "thumbsup".to_string(),
            timestamp: "1234567890.123456".to_string(),
        };
        let json = r##"{
  "channel": "C1234567890",
  "name": "thumbsup",
  "timestamp": "1234567890.123456"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<AddRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[async_std::test]
    async fn test_add() {
        let param = AddRequest {
            channel: "C1234567890".to_string(),
            name: "thumbsup".to_string(),
            timestamp: "1234567890.123456".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true
}"##
            .to_string())
        });

        let response = add(&mock, &param, &"test_token".to_string()).await.unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
