use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum DrinkType {
    PourOver,
    Espresso,
}

#[derive(Serialize, Deserialize)]
struct Coffee {
    region: String,
    roaster: String,
    tasting_notes: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Order {
    customer: String,
    coffee: Coffee,
    drink: DrinkType,
}

#[get("/orders")]
async fn get_orders() -> HttpResponse {
    let orders = vec![Order {
        customer: String::from("Kristi"),
        coffee: Coffee {
            region: String::from("Ethiopia"),
            roaster: String::from("Onyx"),
            tasting_notes: vec![
                String::from("chocolate"),
                String::from("sweet"),
                String::from("citrus"),
            ],
        },
        drink: DrinkType::PourOver,
    }];
    HttpResponse::Ok().json(orders)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo).service(get_orders))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
