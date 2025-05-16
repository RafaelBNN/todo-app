use dotenvy::dotenv;
use std::{env, net::SocketAddr};

mod models;
mod handlers;
mod routes;
mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida no .env");

    let pool = db::init_pool(&database_url).await.expect("Erro ao conectar ao banco de dados");

    let app = routes::create_routes(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
