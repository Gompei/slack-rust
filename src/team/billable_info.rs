use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::team::billing::BillableInfo;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct BillableInfoRequest {
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct BillableInfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub billable_info: Option<HashMap<String, BillableInfo>>,
}

pub async fn billable_info<T>(
    client: &T,
    param: &BillableInfoRequest,
    bot_token: &str,
) -> Result<BillableInfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.billableInfo");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<BillableInfoResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = BillableInfoRequest {
            team_id: Some("T1234567890".to_string()),
            user: Some("W1234567890".to_string()),
        };
        let json = r##"{
  "team_id": "T1234567890",
  "user": "W1234567890"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<BillableInfoRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let mut billable_info = HashMap::new();
        billable_info.insert(
            "U02UCPE1R".to_string(),
            BillableInfo {
                billing_active: true,
            },
        );

        let response = BillableInfoResponse {
            ok: true,
            billable_info: Some(billable_info),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "billable_info": {
    "U02UCPE1R": {
      "billing_active": true
    }
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<BillableInfoResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_billable_info() {
        let param = BillableInfoRequest {
            team_id: Some("T1234567890".to_string()),
            ..Default::default()
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "billable_info": {
    "U02UCPE1R": {
      "billing_active": true
    }
  }
}"##
            .to_string())
        });

        let response = billable_info(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();

        let mut billable_info = HashMap::new();
        billable_info.insert(
            "U02UCPE1R".to_string(),
            BillableInfo {
                billing_active: true,
            },
        );

        let expect = BillableInfoResponse {
            ok: true,
            billable_info: Some(billable_info),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
