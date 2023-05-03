use blocks::SlackMessage;
use serde::Serialize;

pub mod blocks;

pub struct Client {
    reqwest_client: reqwest::Client,
    webhook_url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
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
        self.reqwest_client
            .post(&self.webhook_url)
            .json(msg)
            .send()
            .await?;

        Ok(())
    }
}
