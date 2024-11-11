use crate::{
    domain::request::transfer::{CreateTransferRequest, UpdateTransferRequest},
    state::AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/transfer")]
async fn get_transfers(data: web::Data<AppState>) -> impl Responder {
    match data.di_container.transfer_service.get_transfers().await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch transfers: {}", e),
        })),
    }
}

#[get("/transfer/{id}")]
async fn get_transfer(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .transfer_service
        .get_transfer(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch transfer: {}", e),
        })),
    }
}

#[get("/transfer/users/{id}")]
async fn get_transfer_users(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .transfer_service
        .get_transfer_users(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch transfer users: {}", e),
        })),
    }
}


#[get("/transfer/user/{id}")]
async fn get_transfer_user(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .transfer_service
        .get_transfer_user(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch transfer: {}", e),
        })),
    }
}


#[post("/transfer")]
async fn create_transfer(
    data: web::Data<AppState>,
    body: web::Json<CreateTransferRequest>,
) -> impl Responder {
    match data.di_container.transfer_service.create_transfer(&body).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to create transfer: {}", e),
        })),
    }
}

#[put("/transfer/{id}")]
async fn update_transfer(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<UpdateTransferRequest>,
) -> impl Responder {
    let mut update_request = body.into_inner();
    update_request.transfer_id = id.into_inner();

    match data
        .di_container
        .transfer_service
        .update_transfer(&update_request)
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),

        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
           "message": format!("Failed to update transfer: {}", e),
        })),
    }
}

#[delete("/transfer/{id}")]
async fn delete_transfer(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .transfer_service
        .delete_transfer(id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Transfer deleted successfully",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to delete transfer: {}", e),
        })),
    }
}
