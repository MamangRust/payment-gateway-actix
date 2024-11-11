use crate::{
    domain::request::saldo::{CreateSaldoRequest, UpdateSaldoRequest},
    state::AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/saldos")]
async fn get_saldos(data: web::Data<AppState>) -> impl Responder {
    match data.di_container.saldo_service.get_saldos().await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch saldos: {}", e),
        })),
    }
}

#[get("/saldos/{id}")]
async fn get_saldo(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .saldo_service
        .get_saldo(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch saldo: {}", e),
        })),
    }
}

#[get("/saldos/users/{id}")]
async fn get_saldo_users(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .saldo_service
        .get_saldo_users(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch saldo users: {}", e),
        })),
    }
}

#[get("/saldos/user/{id}")]
async fn get_saldo_user(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .saldo_service
        .get_saldo_user(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch saldo: {}", e),
        })),
    }
}


#[post("/saldos")]
async fn create_saldo(
    data: web::Data<AppState>,
    body: web::Json<CreateSaldoRequest>,
) -> impl Responder {
    match data.di_container.saldo_service.create_saldo(&body).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to create saldo: {}", e),
        })),
    }
}

#[put("/saldos/{id}")]
async fn update_saldo(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<UpdateSaldoRequest>,
) -> impl Responder {
    let mut update_request = body.into_inner();
    update_request.saldo_id = id.into_inner();

    match data
        .di_container
        .saldo_service
        .update_saldo(&update_request)
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),

        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to update saldo: {}", e),
        })),
    }
}

#[delete("/saldos/{id}")]
async fn delete_saldo(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .saldo_service
        .delete_saldo(id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Saldo deleted successfully",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to delete saldo: {}", e),
        })),
    }
}
