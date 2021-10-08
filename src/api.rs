const API_BASE_URL: &str = "https://slack.com/api/";

pub struct Token {
    pub api_key: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenConnectionsResponse {
    pub ok: bool,
    pub url: Option<String>,
    pub error: Option<String>,
}

impl Token {
    pub async fn open_connection(self) -> surf::Result<OpenConnectionsResponse> {
        surf::post(API_BASE_URL.to_string() + "apps.connections.open")
            .header(
                surf::http::headers::AUTHORIZATION,
                format!("Bearer {}", self.api_key),
            )
            .recv_json()
            .await
    }
}
