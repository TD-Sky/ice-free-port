//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::{entity::prelude::*, IntoActiveModel, Set, Unchanged};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "freight_forwarder")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub company_name: String,
    pub telephone_number: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::shipping_order::Entity")]
    ShippingOrder,
}

impl Related<super::shipping_order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShippingOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
pub struct InsertModel {
    pub company_name: String,
    pub telephone_number: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct UpdateModel {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    pub company_name: String,
    pub telephone_number: String,
}

impl IntoActiveModel<ActiveModel> for UpdateModel {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            company_name: Set(self.company_name),
            telephone_number: Set(self.telephone_number),
        }
    }
}
