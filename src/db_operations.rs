use crate::models::{NewTodoItem, NewUser, TodoItem, User};
use crate::schema::todo_items;
use crate::schema::users;
use diesel::prelude::*;
use sha2::{Digest, Sha256};
use std::env;

pub fn establish_connection() -> PgConnection {
    let connection_string =
        env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set");

    PgConnection::establish(&connection_string)
        .unwrap_or_else(|err| panic!("Database error: {}", err))
}

pub fn create_user(conn: &mut PgConnection, username: &str, password: &str) -> QueryResult<User> {
    let mut new_user = NewUser { username, password };

    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let password_hash = format!("{:x}", hasher.finalize());

    new_user.password = &password_hash;

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user(conn: &mut PgConnection, username: &str, password: &str) -> QueryResult<User> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let password_hash = format!("{:x}", hasher.finalize());

    users::table
        .filter(users::username.eq(username))
        .filter(users::password.eq(password_hash))
        .select(User::as_select())
        .first(conn)
}

pub fn create_todo_item(conn: &mut PgConnection, item: &str) {
    let new_todo_item = NewTodoItem { item };

    diesel::insert_into(todo_items::table)
        .values(&new_todo_item)
        .execute(conn)
        .expect("Error creating todo item");
}

pub fn delete_todo_item(conn: &mut PgConnection, id: i32) {
    diesel::delete(todo_items::table.filter(todo_items::id.eq(id)))
        .execute(conn)
        .expect("Error deleting todo item");
}

pub fn get_todo_items(conn: &mut PgConnection) -> QueryResult<Vec<TodoItem>> {
    todo_items::table.select(TodoItem::as_select()).load(conn)
}
