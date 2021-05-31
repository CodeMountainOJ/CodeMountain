use actix_web_validator::{ Json };
use actix_web::{ web::Json as actix_json, Responder };
use super::payload;
use crate::errors;
use crate::db::user::query::get_user_by_email;
use bcrypt::verify;
use crate::jwt::sign::{ generate_accesstoken, generate_refreshtoken };

pub async fn login_handler(req: Json<payload::LoginRequest>) -> Result<impl Responder, errors::Errors> {
    let user = match get_user_by_email(&req.email) {
        Ok(u) => u,
        Err(e) => return Err(e)
    };

    let raw_password = req.password.clone();
    let salted_password = user.password;

    let is_correct_password = match verify(raw_password, &salted_password) {
        Ok(s) => s,
        Err(_) => return Err(errors::Errors::InternalServerError)
    };

    if is_correct_password {
        let access_token = match generate_accesstoken(&user.id) {
            Ok(token) => token,
            Err(_) => return Err(errors::Errors::InternalServerError)
        };

        let refresh_token = match generate_refreshtoken(&user.id) {
            Ok(token) => token,
            Err(_) => return Err(errors::Errors::InternalServerError)
        };

        return Ok(actix_json(payload::LoginTokens {
            access_token,
            refresh_token
        }));
    }

    Err(errors::Errors::BadRequest(String::from("Wrong password!")))
}