use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{barista::DRINK_TYPE_VARIANTS, dal::{get_orders_from_db, get_customer_orders_from_db, insert_order}};

#[derive(Deserialize)]
pub struct OrderInfo {
    pub id: String,
    pub coffee_id: String,
    pub drink_type: String,
    pub customer_id: String,
}

pub async fn get_orders(pool: web::Data<sqlx::Pool<sqlx::Sqlite>>) -> impl Responder {
    match get_orders_from_db(&pool).await {
        Ok(o) => HttpResponse::Ok().json(o),
        Err(..) => HttpResponse::ExpectationFailed().json("oh no"),
    }
}

pub async fn get_customer_orders(pool: web::Data<sqlx::Pool<sqlx::Sqlite>>, path: web::Path<String>) -> impl Responder {
    let customer_id = path.into_inner();
    match get_customer_orders_from_db(&pool, &customer_id).await {
        Ok(o) => HttpResponse::Ok().json(o),
        Err(..) => HttpResponse::ExpectationFailed().json("oh no"),
    }
}

pub async fn get_drink_types() -> impl Responder {
    HttpResponse::Ok().json(DRINK_TYPE_VARIANTS)
}

pub async fn post_order(pool: web::Data<sqlx::Pool<sqlx::Sqlite>>, order: web::Json<OrderInfo>) -> impl Responder {
    match insert_order(&pool, order).await {
        Ok(o) => HttpResponse::Ok().json(o),
        Err(..) => HttpResponse::ExpectationFailed().json("oh no"),
    }
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
