use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::Deserialize;
use tera::Tera;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct FormData {
    tditem: String,
}

#[derive(Clone)]
pub struct ServerState {
    pub tera: Tera,
    pub db_pool: DbPool,
}

pub mod get {
    use super::ServerState;
    use actix_web::{get, web, HttpResponse, Responder};
    use tera::Context;

    #[get("/")]
    pub async fn index(state: web::Data<ServerState>) -> actix_web::Result<impl Responder> {
        let state_clone = state.clone();

        let items = web::block(move || {
            let mut conn = state_clone
                .db_pool
                .get()
                .expect("Couldn't get db connection from pool");

            crate::db_operations::get_todo_items(&mut conn).expect("Error fetching todo items")
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
}

pub mod post {
    use super::{FormData, ServerState};
    use actix_web::{post, web, HttpResponse, Responder};
    use tera::Context;

    #[post("/echo")]
    pub async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    #[post("/submit")]
    pub async fn submit(
        form: web::Form<FormData>,
        state: web::Data<ServerState>,
    ) -> actix_web::Result<impl Responder> {
        let state_clone = state.clone();

        let items = web::block(move || {
            let mut conn = state_clone
                .db_pool
                .get()
                .expect("Couldn't get db connection from pool");

            crate::db_operations::create_todo_item(&mut conn, &form.tditem);
            crate::db_operations::get_todo_items(&mut conn).expect("Error fetching todo items")
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
    pub async fn delete(
        state: web::Data<ServerState>,
        path: web::Path<i32>,
    ) -> actix_web::Result<impl Responder> {
        let state_clone = state.clone();

        let items = web::block(move || {
            let mut conn = state_clone
                .db_pool
                .get()
                .expect("Couldn't get db connection from pool");

            crate::db_operations::delete_todo_item(&mut conn, path.into_inner());
            crate::db_operations::get_todo_items(&mut conn).expect("Error fetching todo items")
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
}
