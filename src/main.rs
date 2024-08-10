use euclidius::{error::Error, route::build_router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;

    let listener = TcpListener::bind("localhost:3000").await?;
    axum::serve(listener, build_router()).await?;

    Ok(())
}
