const API_BASE_URL: &str = "https://slack.com/api/";

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

impl Token {
    pub async fn open_connection(&self) -> surf::Result<OpenConnectionsResponse> {
        surf::post(API_BASE_URL.to_string() + "apps.connections.open")
            .header(
                surf::http::headers::AUTHORIZATION,
                format!("Bearer {}", self.api_key),
            )
            .recv_json()
            .await
    }

    pub async fn open_view(&self, trigger: String) -> surf::Result<APIResponse> {
        // TODO: テストモーダル
        let view_modal = r#"{
          "type": "modal",
          "callback_id": "modal-with-inputs",
          "title": {
            "type": "plain_text",
            "text": "Modal with inputs"
          },
          "blocks": [
            {
              "type": "input",
              "block_id": "multiline",
              "label": {
                "type": "plain_text",
                "text": "Enter your value"
              },
              "element": {
                "type": "plain_text_input",
                "multiline": true,
                "action_id": "mlvalue"
              }
            },
            {
              "block_id": "target_channel",
              "type": "input",
              "optional": true,
              "label": {
                "type": "plain_text",
                "text": "Select a channel to post the result on",
              },
              "element": {
                "action_id": "target_select",
                "type": "conversations_select",
                "response_url_enabled": true,
              },
            }
          ],
          "submit": {
            "type": "plain_text",
            "text": "Submit"
          }
        }"#;

        // クエリ作成
        let query = Query {
            trigger_id: trigger,
            view: String::from(view_modal),
        };

        surf::post(API_BASE_URL.to_string() + "views.open")
            .header(
                surf::http::headers::AUTHORIZATION,
                format!("Bearer {}", self.bot_key),
            )
            .query(&query)?
            .recv_json()
            .await
    }
}
