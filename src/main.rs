use actix_web::{web, App, HttpServer};
use rust_crud::routes::{get::*, post::*, ServerState};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = rust_crud::get_connection_pool();

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ServerState {
                tera: tera.clone(),
                db_pool: db_pool.clone(),
            }))
            .service(index)
            .service(submit)
            .service(delete)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
