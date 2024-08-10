use euclidius::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    println!("Hello, world!");

    Ok(())
}
