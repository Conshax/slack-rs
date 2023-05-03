use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleSlackMessage {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlackMessage {
    pub blocks: Vec<Block>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    type_field: String,

    text: Option<Text>,

    #[serde(rename = "block_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,

    accessory: Option<Accessory>,

    #[serde(default)]
    fields: Vec<Field>,
}

impl Block {
    pub fn new_header(text: String) -> Self {
        Block {
            type_field: "header".to_string(),
            text: Some(Text {
                type_field: "plain_text".to_string(),
                text,
            }),
            block_id: None,
            accessory: None,
            fields: vec![],
        }
    }

    pub fn new_text_section(text: Text) -> Self {
        Block {
            type_field: "section".to_string(),
            text: Some(text),
            block_id: None,
            accessory: None,
            fields: vec![],
        }
    }

    pub fn new_fields_section(fields: Vec<Field>) -> Self {
        Block {
            type_field: "section".to_string(),
            text: None,
            block_id: None,
            accessory: None,
            fields,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessory {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "alt_text")]
    pub alt_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}
