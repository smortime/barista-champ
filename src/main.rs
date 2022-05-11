mod barista;
mod barista_routes;
mod dal;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use barista_routes::{get_drink_types, get_orders, hello, get_customer_orders};
use sqlx::sqlite::SqlitePool;
use std::env;

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
            .service(
                web::scope("/app")
                    .route("/orders", web::get().to(get_orders))
                    .route("/orders/{customer_id}", web::get().to(get_customer_orders))
                    .route("/drinkTypes", web::get().to(get_drink_types)),
            )
            .service(hello)
            .app_data(pool_data.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
