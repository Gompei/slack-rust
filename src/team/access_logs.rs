use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::team::log::{Login, Paging};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct AccessLogsRequest {
    pub before: Option<String>,
    pub count: Option<String>,
    pub page: Option<String>,
    pub team_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct AccessLogsResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub logins: Option<Vec<Login>>,
    pub paging: Option<Paging>,
}

pub async fn access_logs<T>(
    client: &T,
    param: &AccessLogsRequest,
    bot_token: &str,
) -> Result<AccessLogsResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.accessLogs");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<AccessLogsResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = AccessLogsRequest {
            before: Some("1457989166".to_string()),
            count: Some("20".to_string()),
            page: Some("2".to_string()),
            team_id: Some("T1234567890".to_string()),
        };
        let json = r##"{
  "before": "1457989166",
  "count": "20",
  "page": "2",
  "team_id": "T1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<AccessLogsRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = AccessLogsResponse {
            ok: true,
            logins: Some(vec![Login {
                user_id: Some("U45678".to_string()),
                username: Some("alice".to_string()),
                date_first: Some(1422922864),
                date_last: Some(1422922864),
                count: Some(1),
                ip: Some("127.0.0.1".to_string()),
                user_agent: Some("SlackWeb Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.35 Safari/537.36".to_string()),
                isp: Some("BigCo ISP".to_string()),
                country: Some("US".to_string()),
                region: Some( "CA".to_string()),
            }]),
            paging: Some(Paging {
                count: Some(100),
                total: Some(2),
                page: Some(1),
                pages: Some(1),
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "logins": [
    {
      "user_id": "U45678",
      "username": "alice",
      "date_first": 1422922864,
      "date_last": 1422922864,
      "count": 1,
      "ip": "127.0.0.1",
      "user_agent": "SlackWeb Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.35 Safari/537.36",
      "isp": "BigCo ISP",
      "country": "US",
      "region": "CA"
    }
  ],
  "paging": {
    "count": 100,
    "total": 2,
    "page": 1,
    "pages": 1
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<AccessLogsResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_access_logs() {
        let param = AccessLogsRequest {
            before: Some("1457989166".to_string()),
            count: Some("20".to_string()),
            page: Some("2".to_string()),
            team_id: Some("T1234567890".to_string()),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "logins": [
    {
      "user_id": "U45678",
      "username": "alice",
      "date_first": 1422922864,
      "date_last": 1422922864,
      "count": 1,
      "ip": "127.0.0.1",
      "user_agent": "SlackWeb Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.35 Safari/537.36",
      "isp": "BigCo ISP",
      "country": "US",
      "region": "CA"
    }
  ],
  "paging": {
    "count": 100,
    "total": 2,
    "page": 1,
    "pages": 1
  }
}"##
            .to_string())
        });

        let response = access_logs(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = AccessLogsResponse {
            ok: true,
            logins: Some(vec![Login {
                user_id: Some("U45678".to_string()),
                username: Some("alice".to_string()),
                date_first: Some(1422922864),
                date_last: Some(1422922864),
                count: Some(1),
                ip: Some("127.0.0.1".to_string()),
                user_agent: Some("SlackWeb Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.35 Safari/537.36".to_string()),
                isp: Some("BigCo ISP".to_string()),
                country: Some("US".to_string()),
                region: Some( "CA".to_string()),
            }]),
            paging: Some(Paging {
                count: Some(100),
                total: Some(2),
                page: Some(1),
                pages: Some(1),
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
