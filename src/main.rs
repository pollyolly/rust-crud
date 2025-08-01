use axum::http::{ StatusCode, Uri, HeaderValue, Method };
use axum::response::{ IntoResponse };
use axum::{ Extension, Router };
use axum::routing::{ delete, get, post, put };

use tower_http::cors::CorsLayer;
use std::{ fs::{self } };
use std::sync::Arc;

use axum_jwt_auth::{ JwtDecoderState, LocalDecoder};
use jsonwebtoken::{ Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::controller::{
    list_users, 
    get_user_by_id, 
    create_user, 
    update_user, 
    delete_user
};

use crate::user_service::UserService;
use crate::upload_service::{ upload_file, download_file };
use crate::axum_jwt_service::{user_info, login, AppStates};

mod model;
mod controller;
mod user_service;
mod upload_service;
mod axum_jwt_service;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyClaims {
    iat: u64,
    aud: String,
    exp: u64,
}

#[tokio::main]
async fn main() {
    let protocol = "http://";
    let host = "0.0.0.0";
    let port = "3000";
    let host_port = format!("{}:{}",host, port);
    //JWT
    let keys = vec![DecodingKey::from_rsa_pem(include_bytes!("../jwt_keys/public_key.pem")).unwrap()];
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["https://example.com"]);
    let decoder = LocalDecoder::builder()
        .keys(keys)
        .validation(validation)
        .build()
        .unwrap();
    let state = AppStates {
        decoder: JwtDecoderState {
            decoder: Arc::new(decoder),
        },
    };
    //Initialize upload directory
    fs::create_dir_all("./uploads/temp").unwrap();
    //CORS
    let cors = CorsLayer::new()
    .allow_origin(format!("{}{}",protocol,host_port).parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST]);

    println!("Starting Service..!");

    let service = UserService::new().await.unwrap();

    let app = Router::new()
       .route("/user_list/{offset}/{limit}",get(list_users))
       .route("/user_by_id/{id}",get(get_user_by_id))
       .route("/user_create", post(create_user))
       .route("/user_update", put(update_user))
       .route("/user_delete/{id}", delete(delete_user))
       .route("/upload", post(upload_file))
       .route("/download/{filename}", get(download_file))
       .route("/user_info", get(user_info))
       .route("/login", get(login))
       .fallback(handle_missing_route)
       .method_not_allowed_fallback(handle_unsupported_request_method)//Check if method of request is supported. 
                                                            //Simulate error by using POST in a GET request route. 
       .with_state(state)
       .layer(Extension(service))
       .layer(cors);

    let listener = tokio::net::TcpListener::bind(host_port).await.unwrap();

    // println!("Listening on: {}:{}",host,port);
    println!("Listening on: {}",listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

}
async fn handle_unsupported_request_method() -> impl IntoResponse { //We can also do this
// async fn handle_405() -> (StatusCode, String) { //We can do this 
    (StatusCode::NOT_FOUND, format!("This is not allowed!"))
}

async fn handle_missing_route(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
