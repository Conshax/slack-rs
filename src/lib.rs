use serde::{Deserialize, Serialize};

pub mod blocks;

pub struct Client {
    reqwest_client: reqwest::Client,
    webhook_url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Slack error: {0}")]
    SlackError(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlackResponse {
    ok: bool,
    error: Option<String>,
    warning: Option<String>,
}

impl Client {
    pub fn new(webhook_url: String) -> Self {
        let reqwest_client = reqwest::Client::new();

        Self {
            reqwest_client,
            webhook_url,
        }
    }

    pub async fn post_message<T>(&self, msg: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let response = self
            .reqwest_client
            .post(&self.webhook_url)
            .json(msg)
            .send()
            .await?;

        dbg!(&response);

        let response = response.json::<SlackResponse>().await?;

        if response.ok {
            Ok(())
        } else {
            Err(Error::SlackError(
                response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }
}
