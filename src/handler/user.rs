use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{domain::request::{auth::RegisterRequest, user::UpdateUserRequest}, middleware::auth::JwtMiddleware, state::AppState};


#[get("/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder{
    match data.di_container.user_service.get_users().await{
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch users",
        })),
    }
}

#[get("/users/{id}")]
async fn get_user(data: web::Data<AppState>, id: web::Path<i32>, _jwt_guard: JwtMiddleware) -> impl Responder {
    match data
        .di_container
        .user_service
        .find_by_id(id.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
       
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to fetch user",
        })),
    }
}


#[post("/users")]
async fn create_user(
    data: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
    _jwt_guard: JwtMiddleware
) -> impl Responder {
    match data
        .di_container
        .user_service
        .create_user(&body)
        .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to create user",
        })),
    }
}

#[put("/users/{id}")]
async fn update_user(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<UpdateUserRequest>,
    _jwt_guard: JwtMiddleware
) -> impl Responder {
    let mut update_request = body.into_inner();

    update_request.id = Some(id.into_inner());

    match data
        .di_container
        .user_service
        .update_user(&update_request)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "User not found",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to update User",
        })),
    }
}

#[delete("/users/{id}")]
async fn delete_user(data: web::Data<AppState>, id: web::Path<i32>, _jwt_guard: JwtMiddleware) -> impl Responder {
    match data
        .di_container
        .user_service
        .delete_user(id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "User deleted successfully",
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to delete User",
        })),
    }
}
