use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DeleteRequest {
    pub reminder: String,
    pub team_id: Option<String>,
}

pub async fn delete<T>(
    client: &T,
    param: &DeleteRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reminders.delete");
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
        let request = DeleteRequest {
            reminder: "Rm12345678".to_string(),
            team_id: Some("T1234567890".to_string()),
        };
        let json = r##"{
  "reminder": "Rm12345678",
  "team_id": "T1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<DeleteRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[async_std::test]
    async fn test_delete() {
        let param = DeleteRequest {
            reminder: "Rm12345678".to_string(),
            team_id: Some("T1234567890".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true
}"##
            .to_string())
        });

        let response = delete(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
