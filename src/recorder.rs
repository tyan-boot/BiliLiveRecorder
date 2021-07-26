use anyhow::Context;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::bili_api::BiliApi;

enum CtlMsg {
    Stop,

}

pub struct Recoder {
    tx: Option<Sender<CtlMsg>>,
    api: BiliApi,
}

impl Recoder {
    pub fn new() {}

    pub async fn start(&self) -> anyhow::Result<()> {
        if matches!(self.tx, None) {
            let api = self.api.clone();
            let (tx, mut rx) = channel::<CtlMsg>(16);

            tokio::spawn(async move {
                loop {
                    let stream_info = api.get_stream_url().await?;
                    let url = stream_info.durl.first().context("no url")?;
                    let url = url.url.clone();

                    let client = reqwest::Client::new();
                    let mut response = client
                        .get(url)
                        .send()
                        .await?;
                    loop {
                        rx.recv()
                        response.chunk().await
                    }
                    // response.bytes_stream()
                }
            });
        }

        Ok(())
    }

    pub fn stop() {}
}