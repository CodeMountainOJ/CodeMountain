use std::env::var;

pub fn do_env_check() {
    var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be present in the env");
    var("DATABASE_URL").expect("DATABASE_URL must be present in the env");
}