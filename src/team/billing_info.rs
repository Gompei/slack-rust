use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct BillingInfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub plan: Option<String>,
}

pub async fn billing_info<T>(client: &T, bot_token: &str) -> Result<BillingInfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.billing.info");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<BillingInfoResponse>(&result).map_err(Error::SerdeJsonError)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_response() {
        let response = BillingInfoResponse {
            ok: true,
            plan: Some("free".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "plan": "free"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<BillingInfoResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_info() {
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post().returning(|_, _| {
            Ok(r##"{
  "ok": true,
  "plan": "free"
}"##
            .to_string())
        });

        let response = billing_info(&mock, &"test_token".to_string())
            .await
            .unwrap();
        let expect = BillingInfoResponse {
            ok: true,
            plan: Some("free".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
