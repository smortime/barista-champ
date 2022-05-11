mod barista;
mod dal;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::sqlite::SqlitePool;
use std::env;

use crate::{barista::DRINK_TYPE_VARIANTS, dal::get_orders_from_db};

#[get("/orders")]
async fn get_orders(pool: web::Data<sqlx::Pool<sqlx::Sqlite>>) -> HttpResponse {
    match get_orders_from_db(&pool).await {
        Ok(o) => HttpResponse::Ok().json(o),
        Err(..) => HttpResponse::ExpectationFailed().json("oh no"),
    }
}

#[get("/drinkTypes")]
async fn get_drink_types() -> HttpResponse {
    HttpResponse::Ok().json(DRINK_TYPE_VARIANTS)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = match env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(_) => panic!("DATABASE_URL NOT FOUND"),
    };

    let pool = match SqlitePool::connect(&db_url).await {
        Ok(v) => v,
        Err(_) => panic!("CANNOT CONNECT TO DB"),
    };

    let pool_data = web::Data::new(pool);
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
            .service(get_orders)
            .service(get_drink_types)
            .app_data(pool_data.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
