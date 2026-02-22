use crate::feed::{Channel, Entry};
use anyhow::Context;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};

pub fn init() -> Connection {
    let conn = Connection::open("rss.db").unwrap();
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")
        .unwrap();
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS channel (
            id    TEXT PRIMARY KEY,
            title TEXT,
            url   TEXT
        );
        CREATE TABLE IF NOT EXISTS entry (
            id        TEXT PRIMARY KEY,
            channel   TEXT,
            title     TEXT,
            link      TEXT,
            summary   TEXT,
            published TEXT
        );",
    )
    .unwrap();
    conn
}

pub fn store(conn: &mut Connection, channels: &[Channel]) -> anyhow::Result<()> {
    let tx = conn.transaction()?;
    for c in channels {
        tx.execute(
            "INSERT OR REPLACE INTO channel (id, title, url) VALUES (?1, ?2, ?3)",
            params![c.id, c.title, c.url],
        )?;
        for e in &c.entries {
            tx.execute(
                "INSERT OR REPLACE INTO entry (id, channel, title, link, summary, published)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![e.id, c.id, e.title, e.link, e.summary, e.published.to_rfc3339()],
            )?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn load_channels(conn: &Connection) -> anyhow::Result<Vec<Channel>> {
    let mut stmt = conn.prepare_cached(
        "SELECT id, title, url FROM channel ORDER BY title ASC",
    )?;
    let result = stmt
        .query_map([], |r| {
            Ok(Channel { id: r.get(0)?, title: r.get(1)?, url: r.get(2)?, entries: vec![] })
        })?
        .collect::<Result<Vec<_>, _>>()
        .context("load_channels")?;
    Ok(result)
}

fn parse_entry(r: &rusqlite::Row) -> rusqlite::Result<Entry> {
    let published_str: String = r.get(4)?;
    let published = DateTime::parse_from_rfc3339(&published_str)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
            4, rusqlite::types::Type::Text, Box::new(e),
        ))?;
    Ok(Entry { id: r.get(0)?, title: r.get(1)?, link: r.get(2)?, summary: r.get(3)?, published })
}

pub fn load_page(conn: &Connection, offset: usize, limit: usize) -> anyhow::Result<Vec<Entry>> {
    let mut stmt = conn.prepare_cached(
        "SELECT id, title, link, summary, published FROM entry
         ORDER BY published DESC LIMIT ?1 OFFSET ?2",
    )?;
    let result = stmt
        .query_map(params![limit as i64, offset as i64], parse_entry)?
        .collect::<Result<Vec<_>, _>>()
        .context("load_page")?;
    Ok(result)
}

pub fn load_page_for_channel(
    conn: &Connection,
    channel_id: &str,
    offset: usize,
    limit: usize,
) -> anyhow::Result<Vec<Entry>> {
    let mut stmt = conn.prepare_cached(
        "SELECT id, title, link, summary, published FROM entry
         WHERE channel = ?1
         ORDER BY published DESC LIMIT ?2 OFFSET ?3",
    )?;
    let result = stmt
        .query_map(params![channel_id, limit as i64, offset as i64], parse_entry)?
        .collect::<Result<Vec<_>, _>>()
        .context("load_page_for_channel")?;
    Ok(result)
}

pub fn count(conn: &Connection) -> anyhow::Result<usize> {
    let n: i64 = conn.query_row("SELECT COUNT(*) FROM entry", [], |r| r.get(0))?;
    Ok(n as usize)
}

pub fn count_for_channel(conn: &Connection, channel_id: &str) -> anyhow::Result<usize> {
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM entry WHERE channel = ?1",
        params![channel_id],
        |r| r.get(0),
    )?;
    Ok(n as usize)
}
