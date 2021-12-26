use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};

pub async fn delete_photo<T>(client: &T, bot_token: &str) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.deletePhoto");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[async_std::test]
    async fn test_delete_photo() {
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post().returning(|_, _| {
            Ok(r##"{
  "ok": true
}"##
            .to_string())
        });

        let response = delete_photo(&mock, &"test_token".to_string())
            .await
            .unwrap();
        let expect = DefaultResponse {
            ok: true,
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
