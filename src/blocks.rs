use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlackMessage {
    pub text: String,
    pub blocks: Vec<Block>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<Text>,
    #[serde(rename = "block_id")]
    pub block_id: Option<String>,
    pub accessory: Option<Accessory>,
    #[serde(default)]
    pub fields: Vec<Field>,
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
