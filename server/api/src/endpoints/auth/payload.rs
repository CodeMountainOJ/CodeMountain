use actix_web_validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 100))]
    pub password: String
}

#[derive(serde::Deserialize, serde::Serialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub firstname: String,
    #[validate(length(min = 3, max = 20))]
    pub lastname: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 100))]
    pub password: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginTokens {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterReturnPayload {
    pub success: bool
}