#[derive(Debug)]
pub enum Error {
    Dotenvy(dotenvy::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Dotenvy(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<dotenvy::Error> for Error {
    fn from(value: dotenvy::Error) -> Self {
        Self::Dotenvy(value)
    }
}
