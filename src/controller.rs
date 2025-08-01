use axum::extract::Path;
use axum::http::StatusCode;
use axum::{ Extension, Json };

use crate::model::{ Users, UsersInfo };
use crate::user_service::UserService;

pub async fn list_users(service: Extension<UserService>,Path((offset, limit)): Path<(i32, i32)>) -> Result<Json<Vec<Users>>, 
StatusCode
// (StatusCode,String)
> {
 //get user
    match service.list_users(offset, limit).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            eprintln!("{:?}", e); //eprintln! for error printing
            Err(StatusCode::INTERNAL_SERVER_ERROR)
            // Err((StatusCode::NOT_FOUND, format!("This is not allowed!")))
        }
    }
}
 pub async fn get_user_by_id(service: Extension<UserService>,Path(id): Path<i32>) -> Result<Json<Users>, StatusCode> {
//get user by id
    match service.get_users_by_id(id).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            eprintln!("{:?}", e); //eprintln! for error printing
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
 }
 pub async fn create_user(service: Extension<UserService>, Json(user): Json<UsersInfo>) -> StatusCode {
  //create user
    match service.create_user(user).await {
        Ok(_) => StatusCode::OK,
        Err(e)=> {
            eprintln!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
 }
pub async fn update_user(service: Extension<UserService>, Json(user): Json<Users>) -> StatusCode {
 //update user
    match service.update_user(user).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
pub async fn delete_user(service: Extension<UserService>, Path(id): Path<i32>) -> StatusCode {
 //update user
    match service.delete_user(id).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

