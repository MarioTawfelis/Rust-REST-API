use uuid::Uuid;

use diesel::prelude::*;
use diesel::{QueryResult, PgConnection};

use crate::models::user::{User, NewUser, UpdateUser};
use crate::types::email::Email;
use crate::schema::users;

pub fn create_user(
    conn: &mut PgConnection,
    new_user: &NewUser
) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

pub fn get_user_by_id(
    conn: &mut PgConnection, 
    user_id: Uuid
) -> QueryResult<Option<User>> {
    users::table
        .filter(users::id.eq(user_id))
        .first::<User>(conn)
        .optional()
}

pub fn get_user_by_email(
    conn: &mut PgConnection, 
    email: &Email
) -> QueryResult<Option<User>> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .optional()
}

pub fn update_user(
    conn: &mut PgConnection, 
    user_id: Uuid, 
    updated: &UpdateUser
) -> QueryResult<User> {
    diesel::update(users::table.find(user_id))
        .set(updated)
        .get_result(conn)
}

pub fn delete_user(
    conn: &mut PgConnection, 
    user_id: Uuid
) -> QueryResult<usize> {
    diesel::delete(users::table.find(user_id))
        .execute(conn)
}