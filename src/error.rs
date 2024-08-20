use axum::response::IntoResponse;

#[derive(Debug)]
pub enum Error {
    NoReq,
    BadReq(String, String),
    InvalidReq,

    // Errors from other crates, including the standard library.
    Ext(ExtError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoReq => write!(f, "{}", "no request"),
            Self::BadReq(key, value) => write!(f, "bad request: {key}={value}"),
            Self::InvalidReq => write!(f, "{}", "invalid request state"),

            Self::Ext(e) => e.fmt(f),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        format!("{self}").into_response()
    }
}

#[derive(Debug)]
pub enum ExtError {
    Io(std::io::Error),
    EnvVar(std::env::VarError),

    Dotenvy(dotenvy::Error),

    HandlebarsTemplate(handlebars::TemplateError),
    HandlebarsRender(handlebars::RenderError),

    Http(axum::http::Error),

    Sqlx(sqlx::Error),
    SqlxMigrate(sqlx::migrate::MigrateError),
}

impl std::fmt::Display for ExtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::EnvVar(e) => e.fmt(f),

            Self::Dotenvy(e) => e.fmt(f),

            Self::HandlebarsTemplate(e) => e.fmt(f),
            Self::HandlebarsRender(e) => e.fmt(f),

            Self::Sqlx(e) => e.fmt(f),
            Self::SqlxMigrate(e) => e.fmt(f),

            Self::Http(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Ext(ExtError::Io(value))
    }
}

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Self::Ext(ExtError::EnvVar(value))
    }
}

impl From<dotenvy::Error> for Error {
    fn from(value: dotenvy::Error) -> Self {
        Self::Ext(ExtError::Dotenvy(value))
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(value: handlebars::TemplateError) -> Self {
        Self::Ext(ExtError::HandlebarsTemplate(value))
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(value: handlebars::RenderError) -> Self {
        Self::Ext(ExtError::HandlebarsRender(value))
    }
}

impl From<axum::http::Error> for Error {
    fn from(value: axum::http::Error) -> Self {
        Self::Ext(ExtError::Http(value))
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Ext(ExtError::Sqlx(value))
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Ext(ExtError::SqlxMigrate(value))
    }
}
