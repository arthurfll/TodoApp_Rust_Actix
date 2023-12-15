use rusqlite::{Connection,Result,params};
use actix_web::{HttpResponse,web};
use crate::models::keep::Nota;


pub async fn create_nota(nova_nota: web::Json<Nota>) -> Result<HttpResponse> {
    let conn = Connection::open("keeps.sqlite3")?;

    conn.execute(
        "INSERT INTO notas (title, description) VALUES (?1, ?2)",
        params![nova_nota.title, nova_nota.description],
    )?;

    Ok(HttpResponse::Ok().body("Nota criada com sucesso"))
}

pub async fn delete_nota(path: web::Path<i32>) -> Result<HttpResponse> {
    let conn = Connection::open("keeps.sqlite3")?;

    conn.execute(
        "DELETE FROM notas WHERE id = ?1",
        params![path.into_inner()],
    )?;

    Ok(HttpResponse::Ok().body("Nota deletada com sucesso"))
}

pub async fn update_nota(atualizacao: web::Json<Nota>, path: web::Path<i32>) -> Result<HttpResponse> {
    let conn = Connection::open("keeps.sqlite3")?;

    conn.execute(
        "UPDATE notas SET title = ?1, description = ?2 WHERE id = ?3",
        params![atualizacao.title, atualizacao.description, path.into_inner()],
    )?;

    Ok(HttpResponse::Ok().body("Nota atualizada com sucesso"))
}

pub async fn read_nota(path: web::Path<i32>) -> Result<Nota> {
    let conn = Connection::open("keeps.sqlite3")?;
    let mut stmt = conn.prepare("SELECT title, description FROM notas WHERE id = ?1")?;
    let mut rows = stmt.query(params![path.into_inner()])?;

    if let Some(row) = rows.next()? {
        let title: String = row.get(0)?;
        let description: String = row.get(1)?;

        Ok(Nota { title, description })
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}