use actix_web::{HttpServer,App};

pub mod database;
pub mod services;
pub mod models;

pub use services::keep::{create,read,update,delete};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create)
            .service(read)
            .service(update)
            .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
