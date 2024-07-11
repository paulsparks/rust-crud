use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tera::{Context, Tera};

#[get("/")]
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    let page = tera
        .render("index.html", &context)
        .unwrap_or_else(|_| "Template rendering failed".to_string());

    HttpResponse::Ok().content_type("text/html").body(page)
}

#[derive(Deserialize)]
struct FormData {
    tditem: String,
}

#[post("/submit")]
async fn submit(form: web::Form<FormData>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "Paul");
    let page = tera
        .render("index.html", &context)
        .unwrap_or_else(|_| "Template rendering failed".to_string());

    HttpResponse::Ok().content_type("text/html").body(page)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(index)
            .service(submit)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
