pub struct Db {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl Db {
    pub async fn new() -> Self {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost:5432")
            .await
            .unwrap();

        Self { pool }
    }
}
