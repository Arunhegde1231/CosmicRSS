use crate::feed::{Channel, Entry};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rss::Channel as RssChannel;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

const URLS: &[&str] = &[
    "https://feeds.bbci.co.uk/news/world/rss.xml",
    "https://feeds.bbci.co.uk/news/technology/rss.xml",
    "https://feeds.bbci.co.uk/news/science_and_environment/rss.xml",
    "https://www.aljazeera.com/xml/rss/all.xml",
    "https://www.theguardian.com/world/rss",
    "https://www.theguardian.com/uk/technology/rss",
    "https://feeds.npr.org/1001/rss.xml",
    "https://hnrss.org/best",
    "https://feeds.arstechnica.com/arstechnica/index",
    "https://www.theverge.com/rss/index.xml",
    "https://www.wired.com/feed/rss",
    "https://lwn.net/headlines/rss",
    "https://www.phoronix.com/rss.php",
    "https://blog.system76.com/rss.xml",
    "https://www.nasa.gov/news-release/feed/",
    "https://www.sciencedaily.com/rss/top/science.xml",
    "https://this-week-in-rust.org/rss.xml",
    "https://planet.gnome.org/rss20.xml",
];

pub async fn fetch_url(url: &str) -> Result<Channel> {
    let xml = reqwest::get(url).await?.bytes().await?;
    let rss = RssChannel::read_from(&xml[..])?;
    Ok(Channel {
        id: url.to_string(),
        title: rss.title().to_string(),
        url: url.to_string(),
        entries: rss
            .items()
            .iter()
            .filter_map(|i| {
                Some(Entry {
                    id: i.guid()?.value().to_string(),
                    title: i.title()?.to_string(),
                    link: i.link()?.to_string(),
                    summary: i.description().map(|s| s.to_string()),
                    published: i
                        .pub_date()
                        .and_then(|d| DateTime::parse_from_rfc2822(d).ok())?
                        .with_timezone(&Utc),
                })
            })
            .collect(),
    })
}

async fn fetch_all() -> Vec<Channel> {
    futures::future::join_all(URLS.iter().map(|u| fetch_url(u)))
        .await
        .into_iter()
        .flatten()
        .collect()
}


pub async fn sync_loop(
    tx: mpsc::Sender<Vec<Channel>>,
    mut force_rx: mpsc::Receiver<()>,
) {
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut interval = time::interval(Duration::from_secs(600));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = interval.tick() => {},
            _ = force_rx.recv() => {
                while force_rx.try_recv().is_ok() {}
            },
        }

        let channels = fetch_all().await;
        if !channels.is_empty() {
            let _ = tx.send(channels).await;
        }
    }
}
