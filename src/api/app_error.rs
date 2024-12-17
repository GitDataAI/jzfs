
#[derive(Debug)]
pub enum Error{
    TokioError(tokio::io::Error),
    SerdeError(serde_json::Error),
    ActixError(actix_web::Error),
    Other(Box<dyn std::error::Error>),
}

impl From<tokio::io::Error> for Error{
    fn from(err: tokio::io::Error) -> Self{
        Error::TokioError(err)
    }
}
impl From<serde_json::Error> for Error{
    fn from(err: serde_json::Error) -> Self{
        Error::SerdeError(err)
    }
}
impl From<actix_web::Error> for Error{
    fn from(err: actix_web::Error) -> Self{
        Error::ActixError(err)
    }
}

impl From<Box<dyn std::error::Error>> for Error{
    fn from(err: Box<dyn std::error::Error>) -> Self{
        Error::Other(err)
    }
}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TokioError(err) => write!(f, "TokioError: {}", err),
            Error::SerdeError(err) => write!(f, "SerdeError: {}", err),
            Error::ActixError(err) => write!(f, "ActixError: {}", err),
            Error::Other(err) => write!(f, "Other: {}", err),
        }
    }
}