#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Dotenvy(dotenvy::Error),
    Sqlx(sqlx::Error),
    Migrate(sqlx::migrate::MigrateError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::Dotenvy(e) => e.fmt(f),
            Error::Sqlx(e) => e.fmt(f),
            Error::Migrate(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<dotenvy::Error> for Error {
    fn from(value: dotenvy::Error) -> Self {
        Self::Dotenvy(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Migrate(value)
    }
}
