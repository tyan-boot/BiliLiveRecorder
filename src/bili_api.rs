use anyhow::Result;
use serde::{Deserialize};


pub struct BiliApi {
    room_id: u32
}

impl BiliApi {
    pub fn new(room_id: u32) -> BiliApi {
        BiliApi {
            room_id
        }
    }

    pub async fn get_room_info(&self) -> Result<RoomInfo> {
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("https://api.live.bilibili.com/room/v1/Room/get_info?id={}", self.room_id))
            .send()
            .await?;

        let body = response.json::<JsonResp>().await?;

        if body.code != 0 {
            anyhow::bail!("failed get room info");
        }

        let room_info = serde_json::from_value(body.data)?;

        Ok(room_info)
    }

    pub async fn get_stream_url(&self) -> Result<StreamUrlInfo> {
        let client = reqwest::Client::new();

        let url = format!("https://api.live.bilibili.com/room/v1/Room/playUrl?cid={}&quality=4&platform=web", self.room_id);

        let response = client
            .get(&url)
            .send()
            .await?;

        let body = response.json::<JsonResp>().await?;

        if body.code != 0 {
            anyhow::bail!("failed get room_info");
        }

        let stream_ifo = serde_json::from_value::<StreamUrlInfo>(body.data)?;

        Ok(stream_ifo)
    }
}

#[derive(Deserialize, Debug)]
pub struct JsonResp {
    pub code: u32,
    pub data: serde_json::Value,
}

#[derive(Deserialize)]
pub struct RoomInfo {
    pub uid: u32,
    pub short_id: u32,
    pub live_status: u32,
    pub title: String,
    pub area_name: String,
}

#[derive(Deserialize, Debug)]
pub struct StreamUrlInfo {
    pub accept_quality: Vec<String>,
    pub current_qn: u32,
    pub durl: Vec<DUrlInfo>,
}

#[derive(Deserialize, Debug)]
pub struct DUrlInfo {
    pub length: u32,
    pub order: u32,
    pub p2p_type: u32,
    pub stream_type: u32,
    pub url: String,
}

#[cfg(test)]
mod test {
    use crate::bili_api::BiliApi;
    use futures::StreamExt;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn get_room_info() {
        let api = BiliApi::new(358601);

        let room_info = api.get_room_info().await.unwrap();
        assert_eq!(room_info.uid, 24050643);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn get_stream_url() {
        let api = BiliApi::new(358601);

        api.get_stream_url().await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn download() -> anyhow::Result<()> {
        let api = BiliApi::new(358601);

        let room_info = api.get_stream_url().await.unwrap();

        let durl = room_info.durl.first().unwrap();
        let durl = durl.url.clone();
        dbg!(&durl);

        let client = reqwest::Client::new();
        let mut resp = client.get(&durl)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
            .send()
            .await?;
        dbg!(&resp);

        let mut stream = resp.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            dbg!(chunk.len());
        }

        Ok(())
    }
}