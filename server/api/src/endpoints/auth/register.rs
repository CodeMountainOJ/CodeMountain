use actix_web_validator::{ Json };
use actix_web::{ web::Json as actix_json, Responder };
use super::payload;
use crate::errors;
use crate::db::user::query::is_unique;
use crate::db::user::mutation::create_user;
use bcrypt::{hash, DEFAULT_COST};

pub async fn registration_handler(req: Json<payload::RegisterRequest>) -> Result<impl Responder, errors::Errors> {
    let is_unique = match is_unique(
        &req.firstname,
        &req.username,
        &req.email
    ) {
        Ok(u) => u,
        Err(e) => return Err(e)
    };

    if !is_unique {
        return Err(errors::Errors::BadRequest(String::from("Some user data is not unique")));
    }

    let salted_password = match hash(&req.password, DEFAULT_COST) {
        Ok(pwd) => pwd,
        Err(_) => return Err(errors::Errors::InternalServerError)
    };

    let user = create_user(
        &req.firstname,
        &req.lastname, 
        &req.username, 
        &req.email, 
        &salted_password
    );

    match user {
        Ok(_) => Ok(actix_json(payload::RegisterReturnPayload {
            success: true
        })),
        Err(e) => Err(e)
    }
}