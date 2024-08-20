#[derive(Debug,PartialEq,strum::EnumString,strum::Display,serde::Serialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum  ErrorCode{
    NotFound,
    ResourceAlreadyExists,
    PasswordHashFailed,
    DatabaseError,
    SqlBuilderError,
    InvalidInput,
    InternalServeError,
    PermissionDenied,
    Unauthorized
}