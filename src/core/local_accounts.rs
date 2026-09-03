//! 本地服务器账号库（0.5.0 R5）：SQLite 存账号（exe 旁 local_accounts.db），
//! 供「本地服务器」标签页创建账号并按账号拉起客户端。
//! userid 本地自增分配（90000001 起，varint 4 字节内——登录应答模板补丁等长约束见 game_host.rs）。

use anyhow::{anyhow, Result};
use rusqlite::Connection;
use std::path::PathBuf;

/// 本地账号
#[derive(Debug, Clone)]
pub struct LocalAccount {
    pub id: i64,
    pub name: String,
    pub userid: i64,
    pub created_at: String,
}

fn db_path() -> PathBuf {
    std::env::current_exe()
        .map(|e| e.with_file_name("local_accounts.db"))
        .unwrap_or_else(|_| PathBuf::from("local_accounts.db"))
}

fn open() -> Result<Connection> {
    let conn = Connection::open(db_path())?;
    // 并发 create（各自独立 Connection）时等锁而不是立即 BUSY
    conn.busy_timeout(std::time::Duration::from_secs(5))?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS accounts(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            userid INTEGER NOT NULL UNIQUE,
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        );
        CREATE TABLE IF NOT EXISTS meta(
            key TEXT PRIMARY KEY,
            value INTEGER NOT NULL
        );",
    )?;
    Ok(conn)
}

/// 全量列表（按创建序）
pub fn list() -> Result<Vec<LocalAccount>> {
    let conn = open()?;
    let mut stmt = conn.prepare("SELECT id, name, userid, created_at FROM accounts ORDER BY id")?;
    let rows = stmt
        .query_map([], |r| {
            Ok(LocalAccount {
                id: r.get(0)?,
                name: r.get(1)?,
                userid: r.get(2)?,
                created_at: r.get(3)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(rows)
}

/// 创建账号：userid 由 meta 计数器单调分配（事务内原子取号+落号；删号不复用——
/// 旧账号残留的会话/存档/合成凭证会错绑到复用 userid 的新账号上）
pub fn create(name: &str) -> Result<LocalAccount> {
    let name = name.trim();
    if name.is_empty() {
        return Err(anyhow!("账号名不能为空"));
    }
    let mut conn = open()?;
    let tx = conn.transaction()?;
    // 首次使用按现有 MAX 播种（兼容旧库），此后单调递增
    let next_uid: i64 = tx.query_row(
        "INSERT INTO meta(key, value)
         SELECT 'next_userid', COALESCE((SELECT MAX(userid) FROM accounts), 90000000) + 1
         ON CONFLICT(key) DO UPDATE SET value = value + 1
         RETURNING value",
        [],
        |r| r.get(0),
    )?;
    tx.execute(
        "INSERT INTO accounts(name, userid) VALUES(?1, ?2)",
        (name, next_uid),
    )
    .map_err(|e| anyhow!("创建账号失败（重名？）: {e}"))?;
    let id = tx.last_insert_rowid();
    tx.commit()?;
    Ok(LocalAccount {
        id,
        name: name.to_string(),
        userid: next_uid,
        created_at: String::new(),
    })
}

/// 删除账号
pub fn remove(id: i64) -> Result<()> {
    let conn = open()?;
    conn.execute("DELETE FROM accounts WHERE id = ?1", [id])?;
    Ok(())
}

/// 合成凭证（本地 host 放行任意 token；客户端大厅自动登录闸门只需 login=1 + token 形态合法）
pub fn synth_user_info(acc: &LocalAccount) -> crate::core::auth::UserInfo {
    crate::core::auth::UserInfo {
        access_token: String::new(),
        guest_id: String::new(),
        login: 1,
        login_token: String::new(),
        login_token_secret: String::new(),
        login_type: "local".into(),
        token: format!("local-{}", acc.userid),
        token_type: 11, // token_valid() 合法区间 [11,14]
        version: 1,
        userid: Some(acc.userid),
        user_name: Some(acc.name.clone()),
    }
}
