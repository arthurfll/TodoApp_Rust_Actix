use rusqlite::{Connection,Result};



pub async fn abrir_database() -> Result<()> {
    let _conn = Connection::open("keeps.sqlite3")?;
    Ok(())
}

pub async fn criar_tabela() -> Result<()> {
    let conn = Connection::open("keeps.sqlite3")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            description TEXT
        )",
        [],
    )?;
    Ok(())
}