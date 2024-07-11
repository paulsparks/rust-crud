use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::Deserialize;
use tera::{Context, Tera};

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(state: web::Data<ServerState>) -> actix_web::Result<impl Responder> {
    let state_clone = state.clone();

    let items = web::block(move || {
        let mut conn = state_clone
            .db_pool
            .get()
            .expect("Couldn't get db connection from pool");

        rust_crud::db_operations::get_todo_items(&mut conn).expect("Error fetching todo items")
    })
    .await?;

    let mut context = Context::new();
    context.insert("items", &items);
    let page = state
        .tera
        .render("index.html", &context)
        .unwrap_or_else(|_| "Template rendering failed".to_string());

    Ok(HttpResponse::Ok().content_type("text/html").body(page))
}

#[derive(Deserialize)]
struct FormData {
    tditem: String,
}

#[post("/submit")]
async fn submit(
    form: web::Form<FormData>,
    state: web::Data<ServerState>,
) -> actix_web::Result<impl Responder> {
    let state_clone = state.clone();

    let items = web::block(move || {
        let mut conn = state_clone
            .db_pool
            .get()
            .expect("Couldn't get db connection from pool");

        rust_crud::db_operations::create_todo_item(&mut conn, &form.tditem);
        rust_crud::db_operations::get_todo_items(&mut conn).expect("Error fetching todo items")
    })
    .await?;

    let mut context = Context::new();
    context.insert("items", &items);
    let page = state
        .tera
        .render("list.html", &context)
        .unwrap_or_else(|_| "Template rendering failed".to_string());

    Ok(HttpResponse::Ok().content_type("text/html").body(page))
}

#[post("/delete/{id}")]
async fn delete(
    state: web::Data<ServerState>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let state_clone = state.clone();

    let items = web::block(move || {
        let mut conn = state_clone
            .db_pool
            .get()
            .expect("Couldn't get db connection from pool");

        rust_crud::db_operations::delete_todo_item(&mut conn, path.into_inner());
        rust_crud::db_operations::get_todo_items(&mut conn).expect("Error fetching todo items")
    })
    .await?;

    let mut context = Context::new();
    context.insert("items", &items);
    let page = state
        .tera
        .render("list.html", &context)
        .unwrap_or_else(|_| "Template rendering failed".to_string());

    Ok(HttpResponse::Ok().content_type("text/html").body(page))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Clone)]
struct ServerState {
    tera: Tera,
    db_pool: DbPool,
}

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
