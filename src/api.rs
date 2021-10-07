const API_BASE_URL: &str = "https://slack.com/api/";

#[derive(serde::Deserialize, Debug)]
pub struct OpenConnectionsResponse {
    pub ok: bool,
    pub url: Option<String>,
    pub error: Option<String>,
}

pub async fn open_connection(token: &str) -> surf::Result<OpenConnectionsResponse> {
    surf::post(API_BASE_URL.to_string() + "apps.connections.open")
        .header(
            surf::http::headers::AUTHORIZATION,
            format!("Bearer {}", token),
        )
        .recv_json()
        .await
}
