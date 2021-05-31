use actix_web::{error::ResponseError, HttpResponse, http::StatusCode};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Errors {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Access Forbidden")]
    AccessForbidden
}
impl ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        
        #[derive(serde::Serialize)]
        struct Ret {
            pub success: bool,
            pub error: String
        }
        
        match self {
            Errors::InternalServerError => {
                HttpResponse::InternalServerError().json(Ret {
                    success: false,
                    error: String::from("Internal Server Error")
                })
            }

            Errors::BadRequest(ref message) => HttpResponse::BadRequest().json(Ret {
                success: false,
                error: message.clone().to_string()
            }),
            Errors::AccessForbidden => HttpResponse::Forbidden().json(Ret {
                success: false,
                error: String::from("Access forbidden")
            })
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Errors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Errors::BadRequest(_) => StatusCode::BAD_REQUEST,
            Errors::AccessForbidden => StatusCode::FORBIDDEN
        }
    }
}