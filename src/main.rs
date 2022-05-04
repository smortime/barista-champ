use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum BeanStyle {
    Espresso,
    Filtered,
}

#[derive(Serialize, Deserialize)]
enum DrinkType {
    Aeropress,
    IcedCoffee,
    V60,
    Chemex,
    Cappuccino,
    Americano,
    Cortado,
}

const DRINK_TYPE_VARIANTS: &[DrinkType] = &[
    DrinkType::V60,
    DrinkType::Chemex,
    DrinkType::IcedCoffee,
    DrinkType::Aeropress,
    DrinkType::Cappuccino,
    DrinkType::Americano,
    DrinkType::Cortado,
];

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
        drink: DrinkType::V60,
    }];
    HttpResponse::Ok().json(orders)
}

#[get("/drinkTypes")]
async fn get_drink_types() -> HttpResponse {
    HttpResponse::Ok().json(DRINK_TYPE_VARIANTS)
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
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_orders)
            .service(get_drink_types)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
