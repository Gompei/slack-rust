//! Generate a temporary Socket Mode WebSocket URL that your app can connect to in order to receive events and interactive payloads over.    

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ConnectionsOpenResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub url: Option<String>,
}

/// Generate a temporary Socket Mode WebSocket URL that your app can connect to in order to receive events and interactive payloads over.    
/// See: <https://api.slack.com/methods/apps.connections.open>
pub async fn connections_open<T>(
    client: &T,
    app_token: &str,
) -> Result<ConnectionsOpenResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("apps.connections.open");

    client.post(&url, app_token).await.and_then(|result| {
        serde_json::from_str::<ConnectionsOpenResponse>(&result).map_err(Error::SerdeJsonError)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_response() {
        let response = ConnectionsOpenResponse {
            ok: true,
            url: Some("wss://wss-somethiing.slack.com/link/?ticket=12348&app_id=5678".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "url": "wss://wss-somethiing.slack.com/link/?ticket=12348&app_id=5678"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ConnectionsOpenResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_connections_open() {
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post().returning(|_, _| {
            Ok(r##"{
          "ok": true,
          "url": "wss://wss-somethiing.slack.com/link/?ticket=12348&app_id=5678"   
        }"##
            .to_string())
        });

        let response = connections_open(&mock, &"test_token".to_string())
            .await
            .unwrap();
        let expect = ConnectionsOpenResponse {
            ok: true,
            url: Some("wss://wss-somethiing.slack.com/link/?ticket=12348&app_id=5678".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
