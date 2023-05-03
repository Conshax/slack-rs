use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleSlackMessage {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlackMessage {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    type_field: BlockType,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Text>,

    #[serde(rename = "block_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accessory: Option<Accessory>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    Header,
    Section,
    Divider,
    Actions,
    Context,
    File,
    Image,
    Input,
    Video,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextType {
    PlainText,
    Mrkdwn,
}

impl Block {
    pub fn new_header(text: String) -> Self {
        Block {
            type_field: BlockType::Header,
            text: Some(Text {
                type_field: TextType::PlainText,
                text,
                emoji: None,
                verbatim: None,
            }),
            block_id: None,
            accessory: None,
            fields: vec![],
        }
    }

    pub fn new_text_section(text: Text) -> Self {
        Block {
            type_field: BlockType::Section,
            text: Some(text),
            block_id: None,
            accessory: None,
            fields: vec![],
        }
    }

    pub fn new_fields_section(fields: Vec<(Field, Field)>) -> Self {
        let fields = fields.into_iter().fold(vec![], |mut acc, (f1, f2)| {
            acc.push(f1);
            acc.push(f2);
            acc
        });

        Block {
            type_field: BlockType::Section,
            text: None,
            block_id: None,
            accessory: None,
            fields,
        }
    }

    pub fn new_divider() -> Self {
        Block {
            type_field: BlockType::Divider,
            text: None,
            block_id: None,
            accessory: None,
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    #[serde(rename = "type")]
    pub type_field: TextType,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessory {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "alt_text")]
    pub alt_text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    #[serde(rename = "type")]
    pub type_field: TextType,
    pub text: String,
}
