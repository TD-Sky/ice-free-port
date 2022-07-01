use entity::shipping_item;
use sea_orm::{entity::prelude::Date, FromQueryResult};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, FromQueryResult)]
pub struct DetailedItem {
    #[serde_as(as = "DisplayFromStr")]
    pub order_num: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub company_id: i64,
    pub nth: i16,
    pub shipment_date: Date,
    pub quantity: i32,
    pub ton: f64,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Order {
    #[serde_as(as = "DisplayFromStr")]
    pub company_id: i64,
    pub items: Vec<shipping_item::InsertModel>,
}
