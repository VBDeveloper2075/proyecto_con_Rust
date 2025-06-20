use rusqlite::{Connection, Result, params};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct PasswordEntry {
    pub id: String,
    pub fecha: String,
    pub app: String,
    pub url: String,
    pub usuario: String,
    pub mail: String,
    pub con: String, // cifrada
    pub notas: String,
}

impl PasswordEntry {
    pub fn new(app: &str, url: &str, usuario: &str, mail: &str, con: &str, notas: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            fecha: Utc::now().to_rfc3339(),
            app: app.to_string(),
            url: url.to_string(),
            usuario: usuario.to_string(),
            mail: mail.to_string(),
            con: con.to_string(),
            notas: notas.to_string(),
        }
    }
}

pub fn insert_entry(conn: &Connection, entry: &PasswordEntry) -> Result<()> {
    conn.execute(
        "INSERT INTO passwords (id, fecha, app, url, usuario, mail, con, notas) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            entry.id,
            entry.fecha,
            entry.app,
            entry.url,
            entry.usuario,
            entry.mail,
            entry.con,
            entry.notas
        ],
    )?;
    Ok(())
}

pub fn list_entries(conn: &Connection) -> Result<Vec<PasswordEntry>> {
    let mut stmt = conn.prepare("SELECT id, fecha, app, url, usuario, mail, con, notas FROM passwords ORDER BY fecha DESC")?;
    let rows = stmt.query_map([], |row| {
        Ok(PasswordEntry {
            id: row.get(0)?,
            fecha: row.get(1)?,
            app: row.get(2)?,
            url: row.get(3)?,
            usuario: row.get(4)?,
            mail: row.get(5)?,
            con: row.get(6)?,
            notas: row.get(7)?,
        })
    })?;
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}

pub fn search_entries(conn: &Connection, query: &str) -> Result<Vec<PasswordEntry>> {
    let like = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT id, fecha, app, url, usuario, mail, con, notas FROM passwords WHERE app LIKE ?1 OR usuario LIKE ?1 OR mail LIKE ?1 ORDER BY fecha DESC"
    )?;
    let rows = stmt.query_map([like], |row| {
        Ok(PasswordEntry {
            id: row.get(0)?,
            fecha: row.get(1)?,
            app: row.get(2)?,
            url: row.get(3)?,
            usuario: row.get(4)?,
            mail: row.get(5)?,
            con: row.get(6)?,
            notas: row.get(7)?,
        })
    })?;
    let mut entries = Vec::new();
    for entry in rows {
        entries.push(entry?);
    }
    Ok(entries)
}

pub fn delete_entry(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM passwords WHERE id = ?1", [id])?;
    Ok(())
}

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id TEXT PRIMARY KEY,
            fecha TEXT NOT NULL,
            app TEXT NOT NULL,
            url TEXT,
            usuario TEXT,
            mail TEXT,
            con TEXT NOT NULL,
            notas TEXT
        )",
        [],
    )?;
    Ok(conn)
} 