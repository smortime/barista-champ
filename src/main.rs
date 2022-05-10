use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::{env, str::FromStr};
use strum_macros::EnumString;

#[derive(Serialize, Deserialize)]
enum BeanStyle {
    Espresso,
    Filtered,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, EnumString)]
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
    tasting_notes: String,
}

#[derive(Serialize, Deserialize)]
struct Order {
    customer: String,
    coffee: Coffee,
    drink: DrinkType,
}

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

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn get_orders_from_db(pool: &SqlitePool) -> Result<Vec<Order>, sqlx::Error> {
    let orders = sqlx::query!(
        r#"
SELECT cu.name, o.drink_type, c.region_name, c.roaster_name, c.notes
FROM orders o
JOIN 
(SELECT c.id as id, r.name as region_name, ro.name as roaster_name, c.tasting_notes notes
FROM coffees c
JOIN regions r ON r.id = c.region_id
JOIN roasters ro ON ro.id = c.roaster_id) as c ON c.id = o.coffee_id
JOIN customers cu ON cu.id = o.customer_id
        "#,
    )
    .fetch_all(pool)
    .await?;

    let res_orders: Vec<Order> = orders
        .into_iter()
        .map(|o| Order {
            customer: o.name,
            drink: DrinkType::from_str(&o.drink_type.unwrap()).unwrap(),
            coffee: Coffee {
                region: o.region_name,
                roaster: o.roaster_name,
                tasting_notes: o.notes.unwrap(),
            },
        })
        .collect();

    Ok(res_orders)
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
            .service(echo)
            .service(get_orders)
            .service(get_drink_types)
            .app_data(pool_data.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
