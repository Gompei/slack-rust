use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct UninstallRequest {
    pub client_id: String,
    pub client_secret: String,
}

impl UninstallRequest {
    pub fn new(client_id: String, client_secret: String) -> Self {
        UninstallRequest {
            client_id,
            client_secret,
        }
    }
}

pub async fn uninstall<T>(
    client: &T,
    param: &UninstallRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("apps.uninstall");
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
        let request = UninstallRequest {
            client_id: "56579136444.26251006572".to_string(),
            client_secret: "f25b5ceaf8a3c2a2c4f52bb4f0b0499e".to_string(),
        };
        let json = r##"{
  "client_id": "56579136444.26251006572",
  "client_secret": "f25b5ceaf8a3c2a2c4f52bb4f0b0499e"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<UninstallRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[async_std::test]
    async fn test_uninstall() {
        let param = UninstallRequest {
            client_id: "56579136444.26251006572".to_string(),
            client_secret: "f25b5ceaf8a3c2a2c4f52bb4f0b0499e".to_string(),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
          "ok": true
        }"##
            .to_string())
        });

        let response = uninstall(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
