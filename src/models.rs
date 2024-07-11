use crate::schema::{todo_items, users};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = todo_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoItem {
    pub id: i32,
    pub item: String,
}

#[derive(Insertable)]
#[diesel(table_name = todo_items)]
pub struct NewTodoItem<'a> {
    pub item: &'a str,
}
