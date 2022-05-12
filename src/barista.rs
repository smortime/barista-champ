use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};

#[derive(Serialize, Deserialize)]
pub enum BeanStyle {
    Espresso,
    Filtered,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, EnumString)]
pub enum DrinkType {
    Aeropress,
    IcedCoffee,
    V60,
    Chemex,
    Cappuccino,
    Americano,
    Cortado,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, EnumString, Display)]
pub enum DrinkStatus {
    Done,
    Ready,
    Preparing,
}

pub const DRINK_TYPE_VARIANTS: &[DrinkType] = &[
    DrinkType::V60,
    DrinkType::Chemex,
    DrinkType::IcedCoffee,
    DrinkType::Aeropress,
    DrinkType::Cappuccino,
    DrinkType::Americano,
    DrinkType::Cortado,
];

#[derive(Serialize, Deserialize)]
pub struct Coffee {
    pub region: String,
    pub roaster: String,
    pub tasting_notes: String,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub customer: String,
    pub coffee: Coffee,
    pub drink: DrinkType,
    pub status: DrinkStatus,
}
