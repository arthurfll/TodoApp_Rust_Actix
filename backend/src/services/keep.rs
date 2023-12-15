use actix_web::{get,HttpResponse,Responder,web,post,put,delete};

use crate::models::keep::Nota;
use crate::database::{
    keep::{create_nota,read_nota,update_nota,delete_nota}
};



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