use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::team::log::{Log, Paging};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct IntegrationLogsRequest {
    pub app_id: Option<String>,
    pub change_type: Option<String>,
    pub count: Option<String>,
    pub page: Option<String>,
    pub service_id: Option<String>,
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct IntegrationLogsResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub logs: Option<Vec<Log>>,
    pub paging: Option<Paging>,
}

pub async fn integration_logs<T>(
    client: &T,
    param: &IntegrationLogsRequest,
    bot_token: &str,
) -> Result<IntegrationLogsResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.integrationLogs");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<IntegrationLogsResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = IntegrationLogsRequest {
            app_id: Some("xxxxxxxxxxx".to_string()),
            change_type: Some("added".to_string()),
            count: Some("20".to_string()),
            page: Some("2".to_string()),
            service_id: Some("xxxxxxxxxxx".to_string()),
            team_id: Some("T1234567890".to_string()),
            user: Some("W1234567890".to_string()),
        };
        let json = r##"{
  "app_id": "xxxxxxxxxxx",
  "change_type": "added",
  "count": "20",
  "page": "2",
  "service_id": "xxxxxxxxxxx",
  "team_id": "T1234567890",
  "user": "W1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<IntegrationLogsRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = IntegrationLogsResponse {
            ok: true,
            logs: Some(vec![
                Log {
                    service_id: Some(1234567890),
                    service_type: Some("Google Calendar".to_string()),
                    user_id: Some("U1234ABCD".to_string()),
                    user_name: Some("Johnny".to_string()),
                    channel: Some("C1234567890".to_string()),
                    date: Some("1392163200".to_string()),
                    change_type: Some("enabled".to_string()),
                    scope: Some("incoming-webhook".to_string()),
                    ..Default::default()
                },
                Log {
                    app_id: Some("2345678901".to_string()),
                    app_type: Some("Johnny App".to_string()),
                    user_id: Some("U1234ABCD".to_string()),
                    user_name: Some("Johnny".to_string()),
                    channel: Some("C1234567890".to_string()),
                    date: Some("1392163200".to_string()),
                    change_type: Some("enabled".to_string()),
                    scope: Some("incoming-webhook".to_string()),
                    ..Default::default()
                },
            ]),
            paging: Some(Paging {
                count: Some(3),
                total: Some(3),
                page: Some(1),
                pages: Some(1),
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "logs": [
    {
      "service_id": 1234567890,
      "service_type": "Google Calendar",
      "user_id": "U1234ABCD",
      "user_name": "Johnny",
      "channel": "C1234567890",
      "date": "1392163200",
      "change_type": "enabled",
      "scope": "incoming-webhook"
    },
    {
      "app_id": "2345678901",
      "app_type": "Johnny App",
      "user_id": "U1234ABCD",
      "user_name": "Johnny",
      "channel": "C1234567890",
      "date": "1392163200",
      "change_type": "enabled",
      "scope": "incoming-webhook"
    }
  ],
  "paging": {
    "count": 3,
    "total": 3,
    "page": 1,
    "pages": 1
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<IntegrationLogsResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_integration_logs() {
        let param = IntegrationLogsRequest {
            app_id: Some("xxxxxxxxxxx".to_string()),
            change_type: Some("added".to_string()),
            count: Some("20".to_string()),
            page: Some("2".to_string()),
            service_id: Some("xxxxxxxxxxx".to_string()),
            team_id: Some("T1234567890".to_string()),
            user: Some("W1234567890".to_string()),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "logs": [
    {
      "service_id": 1234567890,
      "service_type": "Google Calendar",
      "user_id": "U1234ABCD",
      "user_name": "Johnny",
      "channel": "C1234567890",
      "date": "1392163200",
      "change_type": "enabled",
      "scope": "incoming-webhook"
    },
    {
      "app_id": "2345678901",
      "app_type": "Johnny App",
      "user_id": "U1234ABCD",
      "user_name": "Johnny",
      "channel": "C1234567890",
      "date": "1392163200",
      "change_type": "enabled",
      "scope": "incoming-webhook"
    }
  ],
  "paging": {
    "count": 3,
    "total": 3,
    "page": 1,
    "pages": 1
  }
}"##
            .to_string())
        });

        let response = integration_logs(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();

        let expect = IntegrationLogsResponse {
            ok: true,
            logs: Some(vec![
                Log {
                    service_id: Some(1234567890),
                    service_type: Some("Google Calendar".to_string()),
                    user_id: Some("U1234ABCD".to_string()),
                    user_name: Some("Johnny".to_string()),
                    channel: Some("C1234567890".to_string()),
                    date: Some("1392163200".to_string()),
                    change_type: Some("enabled".to_string()),
                    scope: Some("incoming-webhook".to_string()),
                    ..Default::default()
                },
                Log {
                    app_id: Some("2345678901".to_string()),
                    app_type: Some("Johnny App".to_string()),
                    user_id: Some("U1234ABCD".to_string()),
                    user_name: Some("Johnny".to_string()),
                    channel: Some("C1234567890".to_string()),
                    date: Some("1392163200".to_string()),
                    change_type: Some("enabled".to_string()),
                    scope: Some("incoming-webhook".to_string()),
                    ..Default::default()
                },
            ]),
            paging: Some(Paging {
                count: Some(3),
                total: Some(3),
                page: Some(1),
                pages: Some(1),
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
