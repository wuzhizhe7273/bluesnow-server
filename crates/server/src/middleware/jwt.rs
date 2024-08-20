use crate::context::ServerContext;
use axum::extract::{FromRequestParts, OriginalUri, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use models::r#do::role::Role;
use models::r#do::user::User;
use models::r#do::utils::ToResult;
use sqlx::types::chrono::Utc;
use sqlx::AnyConnection;
use std::sync::OnceLock;
use std::time::Duration;
use uuid::Uuid;

pub static DECODE_HEADER: OnceLock<Validation> = OnceLock::new();
pub static ENCODE_HEADER: OnceLock<Header> = OnceLock::new();

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: u64,
    pub exp: u64,
}

impl Claims {
    pub fn new(sub: Uuid, exp: Duration) -> Self {
        let iat = Utc::now().timestamp() as u64;
        let exp = iat + exp.as_secs();
        Self { sub, exp, iat }
    }
    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> std::result::Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        let decode_header = DECODE_HEADER.get_or_init(|| Validation::new(Algorithm::HS256));
        jsonwebtoken::decode::<Self>(token, key, &decode_header)
    }
    pub fn encode(
        &self,
        key: &EncodingKey,
    ) -> std::result::Result<String, jsonwebtoken::errors::Error> {
        let encode_header = ENCODE_HEADER.get_or_init(|| Header::new(Algorithm::HS256));
        jsonwebtoken::encode(&encode_header, self, key)
    }
}

#[axum::async_trait]
impl FromRequestParts<ServerContext> for Claims {
    type Rejection = result::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerContext,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| result::Error::Unauthorized("Missing Credential".to_string()))?;
        let token = bearer.token();
        let claims = Claims::decode(token, &state.config.jwt.decoding_key())
            .map_err(|_| result::Error::Unauthorized("Invalid Token".to_string()))?
            .claims;
        Ok(claims)
    }
}

async fn check_permission(
    conn: &mut AnyConnection,
    uid: Uuid,
    path: &str,
    method: &str,
) -> result::Result<bool> {
    if User::get_by_uid(conn, uid)
        .await?
        .to_result()?
        .get_related_roles(conn)
        .await?
        .iter()
        .any(|role| role.name == "SuperUser")
    {
        return Ok(true);
    }
    let flag = User::get_by_uid(conn, uid)
        .await?
        .to_result()?
        .get_related_apis(conn)
        .await?
        .iter()
        .any(|api| api.path == path && api.method == method);
    Ok(flag)
}

async fn is_public_api(conn: &mut AnyConnection, path: &str, method: &str) -> result::Result<bool> {
    let flag = Role::get_by_name(conn, "Public")
        .await?
        .to_result()?
        .get_related_api(conn)
        .await?
        .iter()
        .any(|api| api.path == path && method == api.method);
    Ok(flag)
}

pub async fn mid_access_control(
    State(context): State<ServerContext>,
    request: Request,
    next: Next,
) -> result::Result<Response> {
    let (mut parts, body) = request.into_parts();
    let path = parts
        .extract::<OriginalUri>()
        .await
        .and_then(|uri| Ok(uri.path().to_string()))
        .unwrap_or(parts.uri.path().to_string());
    let method = parts.method.as_str().to_string();
    let conn = &mut context.db.acquire().await?;
    // 是否为公开api,如果是则直接放行
    if is_public_api(conn, &path, &method).await? {
        let req = Request::from_parts(parts, body);
        return Ok(next.run(req).await);
    }
    // 解码验证
    let claims = parts.extract_with_state::<Claims, _>(&context).await?;
    if check_permission(conn, claims.sub, &path, &method).await? {
        parts.extensions.insert(claims);
        let req = Request::from_parts(parts, body);
        Ok(next.run(req).await)
    } else {
        Err(result::Error::PermissionDenied)
    }
}
