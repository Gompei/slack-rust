const API_BASE_URL: &str = "https://slack.com/api/";

/// The slack api client
pub struct ApiClient {
    pub token: Token,
}

pub struct Token {
    pub api_key: String,
    pub bot_key: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenConnectionsResponse {
    pub ok: bool,
    pub url: Option<String>,
    pub error: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct APIResponse {
    pub ok: bool,
    pub error: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Query {
    pub trigger_id: String,
    pub view: String,
}

impl ApiClient {
    pub async fn open_connection(&self) -> surf::Result<OpenConnectionsResponse> {
        surf::post(API_BASE_URL.to_string() + "apps.connections.open")
            .header(
                surf::http::headers::AUTHORIZATION,
                format!("Bearer {}", self.token.api_key),
            )
            .recv_json()
            .await
    }
    pub async fn open_view(&self, query: Query) -> surf::Result<APIResponse> {
        surf::post(API_BASE_URL.to_string() + "views.open")
            .header(
                surf::http::headers::AUTHORIZATION,
                format!("Bearer {}", self.token.bot_key),
            )
            .query(&query)?
            .recv_json()
            .await
    }
}
