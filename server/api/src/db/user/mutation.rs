use crate::db::{establish_connection, schema};
use diesel::prelude::*;
use super::model::{ User, NewUser };
use crate::errors::Errors;

pub fn create_user<'a>(
    user_firstname: &'a String,
    user_lastname: &'a String,
    user_username: &'a String,
    user_email: &'a String,
    user_password: &'a String
) -> Result<User, Errors> {
    use schema::users;

    let conn = establish_connection();

    let new_user = NewUser {
        firstname: user_firstname,
        lastname: user_lastname,
        username: user_username,
        email: user_email,
        password: user_password
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&conn) {
            Ok(u) => Ok(u),
            Err(_) => Err(Errors::InternalServerError)
        }
}