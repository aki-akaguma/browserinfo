use anyhow::Result;

#[allow(unused_imports)]
use std::path::PathBuf;

#[allow(unused_imports)]
use dioxus::prelude::*;

#[allow(unused_imports)]
use super::super::{BroInfo, Browser, UserAgent};

#[allow(unused_imports)]
use std::cell::RefCell;

// The database is only available to server code
#[cfg(any(feature = "server", feature = "desktop"))]
thread_local! {
    pub static DB: RefCell<rusqlite::Connection> = {
        let db_path = {
            let mut data_dir = data_dir();
            let db_file = "broinfo.db";
            data_dir.push(db_file);
            data_dir
        };

        // Open the database from the persisted "broinfo.db" file
        let conn = rusqlite::Connection::open(db_path).expect("Failed to open database");

        // Create tables if it doesn't already exist
        create_tables(&conn).unwrap();

        // Return the connection
        RefCell::new(conn)
    };
}

#[cfg(any(feature = "server", feature = "desktop"))]
fn data_dir() -> PathBuf {
    #[allow(unused_assignments)]
    let mut data_dir = PathBuf::from(".");
    #[cfg(feature = "desktop")]
    {
        data_dir = data_dir_on_desktop();
    }
    #[cfg(feature = "server")]
    {
        data_dir = PathBuf::from("/var/local/data/broinfo");
        let _ = std::fs::create_dir_all(&data_dir);
    }
    return data_dir;
}

#[cfg(feature = "desktop")]
fn data_dir_on_desktop() -> PathBuf {
    let mut data_dir = match std::env::home_dir() {
        Some(home) => home,
        None => {
            eprintln!("could NOT get `home_dir()`");
            PathBuf::from(".")
        }
    };
    data_dir.push(".data");
    data_dir.push("broinfo");
    let _ = std::fs::create_dir_all(&data_dir);
    data_dir
}

#[cfg_attr(not(feature = "desktop"), server(input=cbor, output=cbor))]
pub async fn save_user_agent(ua: UserAgent) -> Result<()> {
    let ua_s = ua.get().trim_start_matches('"').trim_end_matches('"');
    //
    #[cfg(feature = "backend_text")]
    write_backend_text("user_agent.txt", ua_s)?;
    //
    DB.with_borrow_mut(|f| {
        let tx = f.transaction()?;
        //
        let user_agent_id = get_or_store_user_agent(&tx, &ua_s)?;
        if user_agent_id == 0 {
            tx.rollback()?;
            return Ok(());
        }
        tx.commit()
    })?;
    //
    dioxus_logger::tracing::debug!("save_user_agent: {ua_s:?}");
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(())
}

#[cfg_attr(not(feature = "desktop"), server(input=cbor, output=cbor))]
pub async fn save_broinfo(broinfo: BroInfo, return_browser: bool) -> Result<Option<Browser>> {
    let user_agent = broinfo.basic.user_agent.clone();
    let referrer = broinfo.basic.referrer.clone();

    let jsinfo_s = toml::to_string(&broinfo.jsinfo).unwrap();
    let jsinfo_ss = jsinfo_s.replace("\n", "<BR>");

    #[cfg(feature = "backend_text")]
    write_backend_text("jsinfo.txt", &info_s)?;
    //
    DB.with_borrow_mut(|f| {
        let tx = f.transaction()?;
        //
        let user_agent_id = get_or_store_user_agent(&tx, user_agent.get())?;
        if user_agent_id == 0 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let referrer_id = get_or_store_referrer(&tx, referrer.get())?;
        if referrer_id == 0 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let jsinfo_id = get_or_store_jsinfo(&tx, &jsinfo_ss)?;
        if jsinfo_id == 0 {
            tx.rollback()?;
            return Ok(());
        }
        //
        tx.execute(
            "INSERT INTO Logs (jsinfo_id, user_agent_id, referrer_id) VALUES (?1, ?2, ?3)",
            &[&jsinfo_id, &user_agent_id, &referrer_id],
        )?;
        //
        tx.commit()
    })?;
    //
    dioxus_logger::tracing::debug!("save_broinfo: {jsinfo_ss:?}");
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    if return_browser {
        Ok(Some(broinfo.to_browser()?))
    } else {
        Ok(None)
    }
}

#[cfg(feature = "backend_text")]
#[allow(dead_code)]
fn write_backend_text(fnm: &str, data: &str) -> Result<()> {
    use std::io::Write;
    //
    // Open file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(fnm)?;
    // And then write a newline to it with data
    let _ = file.write_fmt(format_args!("{data}\n"));
    Ok(())
}

#[allow(dead_code)]
#[cfg(feature = "backend_delay")]
async fn sleep_x(millis: u64) -> Result<()> {
    async_std::task::sleep(std::time::Duration::from_millis(millis)).await;
    Ok(())
}

// Create tables if it doesn't already exist
#[cfg(any(feature = "server", feature = "desktop"))]
fn create_tables(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    // table: `JsInfo`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS JsInfo (
                id INTEGER PRIMARY KEY,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                hash TEXT NOT NULL,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS JsInfo_value ON JsInfo (hash, value);",
    )?;
    {
        let s = "";
        let hash = create_jsinfo_hash(s);
        let hash_s = hash.as_str();
        conn.execute(
            "INSERT INTO JsInfo (hash, value) SELECT * FROM (SELECT ?1, ?2) AS JsInfo
                WHERE NOT EXISTS (SELECT * FROM JsInfo WHERE hash = ?1 AND value = ?2);",
            &[hash_s, s],
        )?;
    }
    // table: `UserAgent`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS UserAgent (
                id INTEGER PRIMARY KEY,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS UserAgent_value ON UserAgent (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO UserAgent (value) SELECT * FROM (SELECT '') AS UserAgent
            WHERE NOT EXISTS (SELECT * FROM UserAgent WHERE value = '');",
    )?;
    // table: `Referrer`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Referrer (
                id INTEGER PRIMARY KEY,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS Referrer_value ON Referrer (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO Referrer (value) SELECT * FROM (SELECT '') AS Referrer
            WHERE NOT EXISTS (SELECT * FROM Referrer WHERE value = '');",
    )?;
    // table: `Logs`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Logs (
                id INTEGER PRIMARY KEY,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                jsinfo_id INTEGER NOT NULL,
                user_agent_id INTEGER NOT NULL,
                referrer_id INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS Logs_jsinfo_id ON Logs (jsinfo_id);
        CREATE INDEX IF NOT EXISTS Logs_user_agent_id ON Logs (user_agent_id);
        CREATE INDEX IF NOT EXISTS Logs_referrer_id ON Logs (referrer_id);",
    )?;
    Ok(())
}

#[cfg(any(feature = "server", feature = "desktop"))]
fn get_or_store_user_agent(tx: &rusqlite::Transaction, user_agent: &str) -> rusqlite::Result<i64> {
    let mut user_agent_id = 0;
    let r: rusqlite::Result<i64> = tx.query_one(
        "SELECT id FROM UserAgent WHERE value = ?1",
        &[user_agent],
        |row| row.get(0),
    );
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute("INSERT INTO UserAgent (value) VALUES (?1)", &[user_agent])?;
        user_agent_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        user_agent_id = id;
    }
    Ok(user_agent_id)
}

#[cfg(any(feature = "server", feature = "desktop"))]
fn get_or_store_referrer(tx: &rusqlite::Transaction, referrer: &str) -> rusqlite::Result<i64> {
    let mut referrer_id = 0;
    let r: rusqlite::Result<i64> = tx.query_one(
        "SELECT id FROM Referrer WHERE value = ?1",
        &[referrer],
        |row| row.get(0),
    );
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute("INSERT INTO Referrer (value) VALUES (?1)", &[referrer])?;
        referrer_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        referrer_id = id;
    }
    Ok(referrer_id)
}

#[cfg(any(feature = "server", feature = "desktop"))]
fn get_or_store_jsinfo(tx: &rusqlite::Transaction, info_s: &str) -> rusqlite::Result<i64> {
    let hash = create_jsinfo_hash(info_s);
    let hash_s = hash.as_str();
    let mut jsinfo_id = 0;
    let r: rusqlite::Result<i64> = tx.query_one(
        "SELECT id FROM JsInfo WHERE hash = ?1 AND value = ?2",
        &[hash_s, info_s],
        |row| row.get(0),
    );
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute(
            "INSERT INTO JsInfo (hash, value) VALUES (?1, ?2)",
            &[hash_s, info_s],
        )?;
        jsinfo_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        jsinfo_id = id;
    }
    Ok(jsinfo_id)
}

#[cfg(any(feature = "server", feature = "desktop"))]
fn create_jsinfo_hash(s: &str) -> String {
    use base64::Engine;

    let hash_bytes = hmac_sha256::Hash::hash(s.as_bytes());
    let hash_base64_s = base64::engine::general_purpose::STANDARD_NO_PAD.encode(hash_bytes);
    hash_base64_s
}
