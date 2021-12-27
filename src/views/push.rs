use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PushRequest {
    pub trigger_id: String,
    pub view: View,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PushResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub view: Option<View>,
}

pub async fn push<T>(
    client: &T,
    param: &PushRequest,
    bot_token: &str,
) -> Result<PushResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("views.push");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<PushResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::block_actions::ActionBlock;
    use crate::block::block_elements::{
        BlockElement, ButtonElement, MultiSelectBlockElement, PlainTextInputBlockElement,
    };
    use crate::block::block_input::InputBlock;
    use crate::block::block_object::{OptionBlockObject, TextBlockObject, TextBlockType};
    use crate::block::blocks::Block;

    use crate::http_client::MockSlackWebAPIClient;
    use crate::views::view::ViewType;

    #[test]
    fn convert_request() {
        let request = PushRequest {
            trigger_id: "12345.98765.abcd2358fdea".to_string(),
            view: View {
                type_filed: Some(ViewType::Modal),
                title: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "Slack Rust Example Modal".to_string(),
                    ..Default::default()
                }),
                submit: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "Submit".to_string(),
                    ..Default::default()
                }),
                blocks: Some(vec![
                    Block::InputBlock(InputBlock {
                        label: TextBlockObject {
                            type_filed: TextBlockType::PlainText,
                            text: "Title".to_string(),
                            ..Default::default()
                        },
                        element: Some(BlockElement::PlainTextInputBlockElement(
                            PlainTextInputBlockElement {
                                action_id: "title".to_string(),
                                placeholder: Some(TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "What do you want to ask of the world?".to_string(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    }),
                    Block::InputBlock(InputBlock {
                        label: TextBlockObject {
                            type_filed: TextBlockType::PlainText,
                            text: "Channel(s)".to_string(),
                            ..Default::default()
                        },
                        element: Some(BlockElement::MultiSelectBlockElement(
                            MultiSelectBlockElement {
                                action_id: "title".to_string(),
                                placeholder: TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "Where should the poll be sent?".to_string(),
                                    ..Default::default()
                                },
                                options: vec![OptionBlockObject {
                                    text: TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "*this is plain_text text*".to_string(),
                                        ..Default::default()
                                    },
                                    value: Some("value-0".to_string()),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    }),
                    Block::ActionBlock(ActionBlock {
                        elements: vec![BlockElement::ButtonElement(ButtonElement {
                            action_id: "add_option".to_string(),
                            text: TextBlockObject {
                                type_filed: TextBlockType::PlainText,
                                text: "Add another option".to_string(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })],
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            },
        };
        let json = r##"{
  "trigger_id": "12345.98765.abcd2358fdea",
  "view": {
    "type": "modal",
    "blocks": [
      {
        "type": "input",
        "label": {
          "type": "plain_text",
          "text": "Title"
        },
        "element": {
          "type": "plain_text_input",
          "action_id": "title",
          "placeholder": {
            "type": "plain_text",
            "text": "What do you want to ask of the world?"
          }
        }
      },
      {
        "type": "input",
        "label": {
          "type": "plain_text",
          "text": "Channel(s)"
        },
        "element": {
          "type": "multi_static_select",
          "placeholder": {
            "type": "plain_text",
            "text": "Where should the poll be sent?"
          },
          "action_id": "title",
          "options": [
            {
              "text": {
                "type": "plain_text",
                "text": "*this is plain_text text*"
              },
              "value": "value-0"
            }
          ]
        }
      },
      {
        "type": "actions",
        "elements": [
          {
            "type": "button",
            "text": {
              "type": "plain_text",
              "text": "Add another option"
            },
            "action_id": "add_option"
          }
        ]
      }
    ],
    "title": {
      "type": "plain_text",
      "text": "Slack Rust Example Modal"
    },
    "submit": {
      "type": "plain_text",
      "text": "Submit"
    }
  }
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<PushRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = PushResponse {
            ok: true,
            view: Some(View {
                type_filed: Some(ViewType::Modal),
                title: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "Slack Rust Example Modal".to_string(),
                    ..Default::default()
                }),
                submit: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "Submit".to_string(),
                    ..Default::default()
                }),
                blocks: Some(vec![
                    Block::InputBlock(InputBlock {
                        label: TextBlockObject {
                            type_filed: TextBlockType::PlainText,
                            text: "Title".to_string(),
                            ..Default::default()
                        },
                        element: Some(BlockElement::PlainTextInputBlockElement(
                            PlainTextInputBlockElement {
                                action_id: "title".to_string(),
                                placeholder: Some(TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "What do you want to ask of the world?".to_string(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    }),
                    Block::InputBlock(InputBlock {
                        label: TextBlockObject {
                            type_filed: TextBlockType::PlainText,
                            text: "Channel(s)".to_string(),
                            ..Default::default()
                        },
                        element: Some(BlockElement::MultiSelectBlockElement(
                            MultiSelectBlockElement {
                                action_id: "title".to_string(),
                                placeholder: TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "Where should the poll be sent?".to_string(),
                                    ..Default::default()
                                },
                                options: vec![OptionBlockObject {
                                    text: TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "*this is plain_text text*".to_string(),
                                        ..Default::default()
                                    },
                                    value: Some("value-0".to_string()),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    }),
                    Block::ActionBlock(ActionBlock {
                        elements: vec![BlockElement::ButtonElement(ButtonElement {
                            action_id: "add_option".to_string(),
                            text: TextBlockObject {
                                type_filed: TextBlockType::PlainText,
                                text: "Add another option".to_string(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })],
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "view": {
    "type": "modal",
    "blocks": [
      {
        "type": "input",
        "label": {
          "type": "plain_text",
          "text": "Title"
        },
        "element": {
          "type": "plain_text_input",
          "action_id": "title",
          "placeholder": {
            "type": "plain_text",
            "text": "What do you want to ask of the world?"
          }
        }
      },
      {
        "type": "input",
        "label": {
          "type": "plain_text",
          "text": "Channel(s)"
        },
        "element": {
          "type": "multi_static_select",
          "placeholder": {
            "type": "plain_text",
            "text": "Where should the poll be sent?"
          },
          "action_id": "title",
          "options": [
            {
              "text": {
                "type": "plain_text",
                "text": "*this is plain_text text*"
              },
              "value": "value-0"
            }
          ]
        }
      },
      {
        "type": "actions",
        "elements": [
          {
            "type": "button",
            "text": {
              "type": "plain_text",
              "text": "Add another option"
            },
            "action_id": "add_option"
          }
        ]
      }
    ],
    "title": {
      "type": "plain_text",
      "text": "Slack Rust Example Modal"
    },
    "submit": {
      "type": "plain_text",
      "text": "Submit"
    }
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<PushResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_push() {
        let param = PushRequest {
            trigger_id: "12345.98765.abcd2358fdea".to_string(),
            view: View {
                type_filed: Some(ViewType::Modal),
                title: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "Slack Rust Example Modal".to_string(),
                    ..Default::default()
                }),
                submit: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "Submit".to_string(),
                    ..Default::default()
                }),
                blocks: Some(vec![
                    Block::InputBlock(InputBlock {
                        label: TextBlockObject {
                            type_filed: TextBlockType::PlainText,
                            text: "Title".to_string(),
                            ..Default::default()
                        },
                        element: Some(BlockElement::PlainTextInputBlockElement(
                            PlainTextInputBlockElement {
                                action_id: "title".to_string(),
                                placeholder: Some(TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "What do you want to ask of the world?".to_string(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    }),
                    Block::InputBlock(InputBlock {
                        label: TextBlockObject {
                            type_filed: TextBlockType::PlainText,
                            text: "Channel(s)".to_string(),
                            ..Default::default()
                        },
                        element: Some(BlockElement::MultiSelectBlockElement(
                            MultiSelectBlockElement {
                                action_id: "title".to_string(),
                                placeholder: TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "Where should the poll be sent?".to_string(),
                                    ..Default::default()
                                },
                                options: vec![OptionBlockObject {
                                    text: TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "*this is plain_text text*".to_string(),
                                        ..Default::default()
                                    },
                                    value: Some("value-0".to_string()),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    }),
                    Block::ActionBlock(ActionBlock {
                        elements: vec![BlockElement::ButtonElement(ButtonElement {
                            action_id: "add_option".to_string(),
                            text: TextBlockObject {
                                type_filed: TextBlockType::PlainText,
                                text: "Add another option".to_string(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })],
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            },
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "view": {
    "type": "modal",
    "blocks": [
      {
        "type": "input",
        "label": {
          "type": "plain_text",
          "text": "Title"
        },
        "element": {
          "type": "plain_text_input",
          "action_id": "title",
          "placeholder": {
            "type": "plain_text",
            "text": "What do you want to ask of the world?"
          }
        }
      },
      {
        "type": "input",
        "label": {
          "type": "plain_text",
          "text": "Channel(s)"
        },
        "element": {
          "type": "multi_static_select",
          "placeholder": {
            "type": "plain_text",
            "text": "Where should the poll be sent?"
          },
          "action_id": "title",
          "options": [
            {
              "text": {
                "type": "plain_text",
                "text": "*this is plain_text text*"
              },
              "value": "value-0"
            }
          ]
        }
      },
      {
        "type": "actions",
        "elements": [
          {
            "type": "button",
            "text": {
              "type": "plain_text",
              "text": "Add another option"
            },
            "action_id": "add_option"
          }
        ]
      }
    ],
    "title": {
      "type": "plain_text",
      "text": "Slack Rust Example Modal"
    },
    "submit": {
      "type": "plain_text",
      "text": "Submit"
    }
  }
}"##
            .to_string())
        });

        let response = push(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = PushResponse {
            ok: true,
            view: Some(View {
                type_filed: Some(ViewType::Modal),
                title: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "Slack Rust Example Modal".to_string(),
                    ..Default::default()
                }),
                submit: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "Submit".to_string(),
                    ..Default::default()
                }),
                blocks: Some(vec![
                    Block::InputBlock(InputBlock {
                        label: TextBlockObject {
                            type_filed: TextBlockType::PlainText,
                            text: "Title".to_string(),
                            ..Default::default()
                        },
                        element: Some(BlockElement::PlainTextInputBlockElement(
                            PlainTextInputBlockElement {
                                action_id: "title".to_string(),
                                placeholder: Some(TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "What do you want to ask of the world?".to_string(),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    }),
                    Block::InputBlock(InputBlock {
                        label: TextBlockObject {
                            type_filed: TextBlockType::PlainText,
                            text: "Channel(s)".to_string(),
                            ..Default::default()
                        },
                        element: Some(BlockElement::MultiSelectBlockElement(
                            MultiSelectBlockElement {
                                action_id: "title".to_string(),
                                placeholder: TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "Where should the poll be sent?".to_string(),
                                    ..Default::default()
                                },
                                options: vec![OptionBlockObject {
                                    text: TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "*this is plain_text text*".to_string(),
                                        ..Default::default()
                                    },
                                    value: Some("value-0".to_string()),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    }),
                    Block::ActionBlock(ActionBlock {
                        elements: vec![BlockElement::ButtonElement(ButtonElement {
                            action_id: "add_option".to_string(),
                            text: TextBlockObject {
                                type_filed: TextBlockType::PlainText,
                                text: "Add another option".to_string(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })],
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
