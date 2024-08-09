use euclidius::error::Error;

fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    println!("Hello, world!");

    Ok(())
}
