use tokio::net::TcpListener;

use euclidius::{app::App, db, result::Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let database_url = std::env::var("DATABASE_URL")?;
    let database_conn = db::connect_or_setup(&database_url).await?;
    let app = App::new(database_conn)?;

    let server_url = std::env::var("SERVER_URL")?;
    let listener = TcpListener::bind(&server_url).await?;
    println!("Listening on: {server_url}");

    let router = app.into_router();
    axum::serve(listener, router).await?;
    Ok(())
}
