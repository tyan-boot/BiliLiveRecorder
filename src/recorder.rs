use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::bili_api::BiliApi;
use anyhow::Context;

enum CtlMsg {
    Stop,

}

pub struct Recoder {
    tx: Option<Sender<CtlMsg>>,
    api: BiliApi,
}

impl Recoder {
    pub fn new() {

    }

    pub async fn start(&self) -> anyhow::Result<()> {
        if matches!(self.tx, None) {
            let stream_info = self.api.get_stream_url().await?;
            let url = stream_info.durl.first().context("no url")?;
            let url = url.url.clone();

            let (tx, rx) = channel::<CtlMsg>(16);
            tokio::spawn(async move {
                let client = reqwest::Client::new();
                let response = client
                    .get(url)
                    .send()
                    .await?;

                // response.bytes_stream()
                loop {

                }
            });
        }

        Ok(())
    }

    pub fn stop() {

    }
}