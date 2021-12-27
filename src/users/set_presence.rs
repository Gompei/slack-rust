use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SetPresenceRequest {
    pub presence: String,
}

pub async fn set_presence<T>(
    client: &T,
    param: &SetPresenceRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.setPresence");
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
        let request = SetPresenceRequest {
            presence: "away".to_string(),
        };
        let json = r##"{
  "presence": "away"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<SetPresenceRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[async_std::test]
    async fn test_set_presence() {
        let param = SetPresenceRequest {
            presence: "away".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true
}"##
            .to_string())
        });

        let response = set_presence(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
