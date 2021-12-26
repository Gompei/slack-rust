use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct UnarchiveRequest {
    pub channel: String,
}

pub async fn unarchive<T>(
    client: &T,
    param: &UnarchiveRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.unarchive");
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
        let request = UnarchiveRequest {
            channel: "C1234567890".to_string(),
        };
        let json = r##"{
  "channel": "C1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<UnarchiveRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[async_std::test]
    async fn test_unarchive() {
        let param = UnarchiveRequest {
            channel: "C1234567890".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true
}"##
            .to_string())
        });

        let response = unarchive(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
