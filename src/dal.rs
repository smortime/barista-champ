use std::str::FromStr;

use actix_web::web;
use sqlx::SqlitePool;

use crate::{barista::{Coffee, DrinkType, Order}, barista_routes::OrderInfo};

pub async fn get_orders_from_db(pool: &SqlitePool) -> Result<Vec<Order>, sqlx::Error> {
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

pub async fn get_customer_orders_from_db(pool: &SqlitePool, customer_id: &str) -> Result<Vec<Order>, sqlx::Error> {
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
WHERE cu.id = $1
        "#, customer_id
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

pub async fn insert_order(pool: &SqlitePool, order: web::Json<OrderInfo>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO orders (id, coffee_id, drink_type, customer_id)
VALUES
        ($1, $2, $3, $4)
        "#, order.id, order.coffee_id, order.drink_type, order.customer_id
    )
    .execute(pool)
    .await?;

    Ok(())
}