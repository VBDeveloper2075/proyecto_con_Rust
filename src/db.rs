use rusqlite::{params, Connection, Result, NO_PARAMS};
use anyhow::Context;
use std::path::Path;

// Estructura para representar una contraseña almacenada
#[derive(Debug)]
pub struct Password {
    pub id: Option<i64>,
    pub service: String,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    // Inicializa la base de datos, creando el archivo si no existe
    pub fn new<P: AsRef<Path>>(db_path: P) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)
            .context("No se pudo abrir o crear la base de datos")?;
        
        // Crear tabla si no existe
        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                service TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                notes TEXT
            )",
            NO_PARAMS,
        ).context("No se pudo crear la tabla de contraseñas")?;

        Ok(Database { conn })
    }

    // Añade una nueva contraseña a la base de datos
    pub fn add_password(&self, password: &Password) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO passwords (service, username, password, notes) VALUES (?1, ?2, ?3, ?4)",
            params![password.service, password.username, password.password, password.notes],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }

    // Obtiene todas las contraseñas
    pub fn get_all_passwords(&self) -> Result<Vec<Password>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, service, username, password, notes FROM passwords ORDER BY service"
        )?;
        
        let password_iter = stmt.query_map(NO_PARAMS, |row| {
            Ok(Password {
                id: Some(row.get(0)?),
                service: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                notes: row.get(4)?,
            })
        })?;
        
        let mut passwords = Vec::new();
        for password in password_iter {
            passwords.push(password?);
        }
        
        Ok(passwords)
    }

    // Busca contraseñas que coincidan con un servicio
    pub fn search_passwords(&self, query: &str) -> Result<Vec<Password>> {
        let query = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, service, username, password, notes FROM passwords 
             WHERE service LIKE ?1 OR username LIKE ?1 ORDER BY service"
        )?;
        
        let password_iter = stmt.query_map(params![query], |row| {
            Ok(Password {
                id: Some(row.get(0)?),
                service: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                notes: row.get(4)?,
            })
        })?;
        
        let mut passwords = Vec::new();
        for password in password_iter {
            passwords.push(password?);
        }
        
        Ok(passwords)
    }

    // Obtiene una contraseña por su ID
    pub fn get_password(&self, id: i64) -> Result<Option<Password>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, service, username, password, notes FROM passwords WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query(params![id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Password {
                id: Some(row.get(0)?),
                service: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                notes: row.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    // Actualiza una contraseña existente
    pub fn update_password(&self, password: &Password) -> Result<()> {
        if let Some(id) = password.id {
            self.conn.execute(
                "UPDATE passwords SET service = ?1, username = ?2, password = ?3, notes = ?4 WHERE id = ?5",
                params![password.service, password.username, password.password, password.notes, id],
            )?;
            Ok(())
        } else {
            Err(rusqlite::Error::InvalidParameterName("La contraseña no tiene ID".into()))
        }
    }

    // Elimina una contraseña por su ID
    pub fn delete_password(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM passwords WHERE id = ?1", params![id])?;
        Ok(())
    }
}
