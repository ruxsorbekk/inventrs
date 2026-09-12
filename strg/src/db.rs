use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
pub async fn establish_connection() -> Result<Pg, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@127.0.0.1:5432/inventrs").await?;
    Ok(())
}