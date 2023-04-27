use blocks::SlackMessage;

mod blocks;

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
    pub async fn new(webhook_url: String) -> Self {
        let reqwest_client = reqwest::Client::new();

        Self {
            reqwest_client,
            webhook_url,
        }
    }

    pub async fn post_message(&self, msg: &SlackMessage) -> Result<(), Error> {
        self.reqwest_client
            .post(&self.webhook_url)
            .json(msg)
            .send()
            .await?;

        Ok(())
    }
}
