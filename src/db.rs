use rusqlite::{Connection, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Database {
    conn: Arc<Mutex<Connection>>, // shared mutex connection for per-thread safety
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                email TEXT
            )",
            [],
        )?;

        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn add_user(&self, name: &str, email: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO users (name, email) VALUES (?1, ?2)",
            [name, email],
        )?;
        Ok(())
    }
}