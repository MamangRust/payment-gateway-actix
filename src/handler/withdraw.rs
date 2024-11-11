use crate::{
    domain::request::withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest},
    state::AppState,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/withdraw")]
async fn get_withdraws(data: web::Data<AppState>) -> impl Responder {
    match data.di_container.withdraw_service.get_withdraws().await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch withdraws: {}", e),
        })),
    }
}

#[get("/withdraw/{id}")]
async fn get_withdraw(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .withdraw_service
        .get_withdraw(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch withdraw: {}", e),
        })),
    }
}

#[get("/withdraw/users/{id}")]
async fn get_withdraw_users(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .withdraw_service
        .get_withdraw_users(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch withdraw users: {}", e),
        })),
    }
}


#[get("/transfer/user/{id}")]
async fn get_withdraw_user(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .withdraw_service
        .get_withdraw_user(id.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to fetch withdraw: {}", e),
        })),
    }
}


#[post("/transfer")]
async fn create_withdraw(
    data: web::Data<AppState>,
    body: web::Json<CreateWithdrawRequest>,
) -> impl Responder {
    match data.di_container.withdraw_service.create_withdraw(&body).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to create withdraw: {}", e),
        })),
    }
}

#[put("/withdraw/{id}")]
async fn update_withdraw(
    data: web::Data<AppState>,
    id: web::Path<i32>,
    body: web::Json<UpdateWithdrawRequest>,
) -> impl Responder {
    let mut update_request = body.into_inner();
    update_request.withdraw_id = id.into_inner();

    match data
        .di_container
        .withdraw_service
        .update_withdraw(&update_request)
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),

        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
           "message": format!("Failed to update withdraw: {}", e),
        })),
    }
}

#[delete("/withdraw/{id}")]
async fn delete_withdraw(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match data
        .di_container
        .withdraw_service
        .delete_withdraw(id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Withdraw deleted successfully",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Failed to delete Withdraw: {}", e),
        })),
    }
}
