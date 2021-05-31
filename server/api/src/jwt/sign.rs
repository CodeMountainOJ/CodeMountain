use super::claims::{AccessToken, RefreshToken};
use jsonwebtoken::{encode, errors::Result, EncodingKey, Header};

fn sign(claims: impl serde::Serialize) -> Result<String> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET_KEY")
                .expect("JWT_SECRET_KEY should be in the env")
                .as_bytes(),
        ),
    )
}

pub fn generate_accesstoken(uid: &i32) -> Result<String> {
    let claims = AccessToken {
        uid: uid.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp(),
    };

    sign(claims)
}

pub fn generate_refreshtoken(uid: &i32) -> Result<String> {
    let claims = RefreshToken {
        uid: uid.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp(),
    };

    sign(claims)
}
