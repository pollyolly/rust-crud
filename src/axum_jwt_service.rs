use axum::{
    extract::{FromRef, State},
    response::{IntoResponse, Response},
    Json
};
use axum_jwt_auth::{Claims, JwtDecoderState };
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header };
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyClaims {
    iat: u64,
    aud: String,
    exp: u64,
}

#[derive(Clone, FromRef)]
pub struct AppStates {
    pub decoder: JwtDecoderState<MyClaims>,
}

// If using `axum::debug_handler`, You will have to include the state parameter in the function signature. Otherwise, you can remove it.
// https://github.com/tokio-rs/axum/discussions/2240#discussioncomment-7100590
#[axum::debug_handler]
pub async fn user_info(Claims(claims): Claims<MyClaims>, State(_state): State<AppStates>) -> Response {
    Json(claims).into_response()
}

pub async fn login() -> Response {
    let key = EncodingKey::from_rsa_pem(include_bytes!("../jwt_keys/private_key.pem")).unwrap();
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("test".to_string());

    let exp = Utc::now() + Duration::hours(5);
    let claims = MyClaims {
        iat: 1234567890,
        aud: "https://example.com".to_string(),
        exp: exp.timestamp() as u64,
    };

    let token = encode::<MyClaims>(&header, &claims, &key).unwrap();

    token.into_response()
}
