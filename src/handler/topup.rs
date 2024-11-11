use crate::{
    domain::request::topup::{CreateTopupRequest, UpdateTopupRequest},
    state::AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/topups")]
async fn get_topups(data: web::Data<AppState>) -> impl Responder {
    match data.di_container.topup_service.get_topups().await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch topups: {}", e),
        })),
    }
}

#[get("/topups/{id}")]
async fn get_topup(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .topup_service
        .get_topup(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch topup: {}", e),
        })),
    }
}

#[get("/topups/users/{id}")]
async fn get_topup_users(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .topup_service
        .get_topup_users(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch topup users: {}", e),
        })),
    }
}

#[get("/topups/user/{id}")]
async fn get_topup_user(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .topup_service
        .get_topup_user(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch topup: {}", e),
        })),
    }
}


#[post("/topups")]
async fn create_topup(
    data: web::Data<AppState>,
    body: web::Json<CreateTopupRequest>,
) -> impl Responder {
    match data.di_container.topup_service.create_topup(&body).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to create topup: {}", e),
        })),
    }
}

#[put("/topups/{id}")]
async fn update_topup(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<UpdateTopupRequest>,
) -> impl Responder {
    let mut update_request = body.into_inner();
    update_request.topup_id = id.into_inner();

    match data
        .di_container
        .topup_service
        .update_topup(&update_request)
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),

        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
           "message": format!("Failed to update topup: {}", e),
        })),
    }
}

#[delete("/topups/{id}")]
async fn delete_topup(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .topup_service
        .delete_topup(id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Topup deleted successfully",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to delete topup: {}", e),
        })),
    }
}
