use sqlx::{postgres::PgPoolOptions, pool::Pool, Postgres, Error};
use std::env;

pub async fn get_pool() -> Result<Pool<Postgres>, Error> {
    let db_url = env::var("DATABASE_URL").unwrap();
    let db_pool_size = env::var("DB_POOL_SIZE").unwrap().parse::<u32>().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(db_pool_size)
        .connect(&db_url).await;

    match pool {
        Ok(p) => Ok(p),
        Err(e) => Err(e)
    }
}