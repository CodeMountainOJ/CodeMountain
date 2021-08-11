#![allow(clippy::result_unit_err)]
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use redis::Commands;

pub type RedisPool = Pool<RedisConnectionManager>;

/// ## Creates a Redis connection pool and returns it
///
/// Example:
/// ```rust
/// let pool = create_redis_pool();
/// ```
pub fn create_redis_pool() -> RedisPool {
    let redis_con_url = std::env::var("REDIS_CON").unwrap();

    let manager = RedisConnectionManager::new(redis_con_url)
        .expect("Failed to make Redis connection manager");
    Pool::builder()
        .build(manager)
        .expect("Failed to establish Redis connection")
}

/// ## Gets a connection from the connection pool
pub fn get_con(pool: &RedisPool) -> Result<PooledConnection<RedisConnectionManager>, ()> {
    match pool.get() {
        Ok(con) => Ok(con),
        Err(_) => Err(()),
    }
}

/// ## Sets given key and value in the redis db
///
/// Expiry time in seconds
///
/// Example:
/// ```rust
/// set_data(conn_poll, "robert", "funny", 1);
/// ```
pub fn set_data(pool: &RedisPool, key: &str, value: &str, expire_time: usize) -> Result<(), ()> {
    let mut conn = match get_con(pool) {
        Ok(con) => con,
        Err(_) => return Err(()),
    };

    let _data: String = match conn.set(key, value) {
        Ok(_d) => _d,
        Err(_) => return Err(()),
    };

    let _: String = match conn.expire(key, expire_time) {
        Ok(_d) => _d,
        Err(e) => match e.kind() {
            redis::ErrorKind::TypeError => String::new(), // ignore this
            _ => return Err(()),
        },
    };

    Ok(())
}

/// ## Gets value of given key from redis db
pub fn get_data(pool: &RedisPool, key: &str) -> Result<String, ()> {
    let mut conn = match get_con(pool) {
        Ok(con) => con,
        Err(_) => return Err(()),
    };

    let data: String = match conn.get(key) {
        Ok(data) => data,
        Err(e) => {
            return {
                match e.kind() {
                    redis::ErrorKind::TypeError => Ok(String::new()), // means there's no data because we will not store anything other than strings
                    _ => Err(()),
                }
            }
        }
    };

    Ok(data)
}
