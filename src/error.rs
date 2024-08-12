use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    EnvVar(std::env::VarError),
    Dotenvy(dotenvy::Error),
    Sqlx(sqlx::Error),
    Migrate(sqlx::migrate::MigrateError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::EnvVar(e) => e.fmt(f),
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

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Self::EnvVar(value)
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

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::new(format!("{self}").into())
    }
}
