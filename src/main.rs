use euclidius::{app::App, error::Error};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let app = App::new(euclidius::validate_database_and_connect().await?);

    let listener = TcpListener::bind("localhost:3000").await?;
    axum::serve(listener, app.build_router()).await?;

    Ok(())
}
