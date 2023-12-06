// Importações =====================================================================

use actix_web::{HttpServer,App,get,HttpResponse,Responder,web,post,put,delete};
use serde::{Deserialize,Serialize,};
use rusqlite::{Connection,Result,params};

// Models ==========================================================================

#[derive(Deserialize,Serialize)]
pub struct Nota {
    pub title       : String ,
    pub description : String ,
}

// Database ========================================================================

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


pub async fn show_all_notes() -> Result<Vec<Nota>> {
    let conn = Connection::open("keeps.sqlite3")?;

    let mut stmt = conn.prepare("SELECT title, description FROM notas")?;
    let notes_iter = stmt.query_map([], |row| {
        Ok(Nota {
            title: row.get(0)?,
            description: row.get(1)?,
        })
    })?;
    let notes: Result<Vec<Nota>> = notes_iter.collect();
    notes
}

// crud ================

async fn create_nota(nova_nota: web::Json<Nota>) -> Result<HttpResponse> {
    let conn = Connection::open("keeps.sqlite3")?;

    conn.execute(
        "INSERT INTO notas (title, description) VALUES (?1, ?2)",
        params![nova_nota.title, nova_nota.description],
    )?;

    Ok(HttpResponse::Ok().body("Nota criada com sucesso"))
}

async fn delete_nota(path: web::Path<i32>) -> Result<HttpResponse> {
    let conn = Connection::open("keeps.sqlite3")?;

    conn.execute(
        "DELETE FROM notas WHERE id = ?1",
        params![path.into_inner()],
    )?;

    Ok(HttpResponse::Ok().body("Nota deletada com sucesso"))
}

async fn update_nota(atualizacao: web::Json<Nota>, path: web::Path<i32>) -> Result<HttpResponse> {
    let conn = Connection::open("keeps.sqlite3")?;

    conn.execute(
        "UPDATE notas SET title = ?1, description = ?2 WHERE id = ?3",
        params![atualizacao.title, atualizacao.description, path.into_inner()],
    )?;

    Ok(HttpResponse::Ok().body("Nota atualizada com sucesso"))
}

async fn read_nota(path: web::Path<i32>) -> Result<Nota> {
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


// Services ========================================================================

#[get("/")]
pub async fn home() -> impl Responder {
    if let Ok(notes) = show_all_notes().await {
        let notes_json = serde_json::to_string(&notes).unwrap();
        HttpResponse::Ok().body(notes_json)
    } else {
        HttpResponse::InternalServerError().body("Erro ao recuperar as notas")
    }
}


#[post("/create")]
pub async fn create(nova_nota: web::Json<Nota>) -> impl Responder {
    if let Ok(_) = create_nota(nova_nota).await {
        HttpResponse::Ok().body("Nota criada com sucesso")
    } else {
        HttpResponse::InternalServerError().body("Erro ao criar a nota")
    }
}
#[get("/read/{id}")]
pub async fn read(path: web::Path<i32>) -> impl Responder {
    match read_nota(path).await {
        Ok(nota) => HttpResponse::Ok().json(nota),
        Err(_) => HttpResponse::NotFound().body("Nota não encontrada"),
    }
}

#[put("/update/{id}")]
pub async fn update(atualizacao: web::Json<Nota>, path: web::Path<i32>) -> impl Responder {
    if let Ok(_) = update_nota(atualizacao, path).await {
        HttpResponse::Ok().body("Nota atualizada com sucesso")
    } else {
        HttpResponse::InternalServerError().body("Erro ao atualizar a nota")
    }
}

#[delete("/delete/{id}")]
pub async fn delete(path: web::Path<i32>) -> impl Responder {
    if let Ok(_) = delete_nota(path).await {
        HttpResponse::Ok().body("Nota deletada com sucesso")
    } else {
        HttpResponse::InternalServerError().body("Erro ao deletar a nota")
    }
}

// Main ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(create)
            .service(read)
            .service(update)
            .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
