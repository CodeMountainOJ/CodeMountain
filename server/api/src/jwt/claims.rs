use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub uid: i32,
    pub exp: i64
}

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub uid: i32,
    pub exp: i64
}