use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct BillingInfoResponse {
    pub ok: bool,
    pub error: Option<String>,
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
