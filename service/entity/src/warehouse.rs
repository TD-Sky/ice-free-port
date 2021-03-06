//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::{entity::prelude::*, IntoActiveModel, Set, Unchanged};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "warehouse")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub house_name: String,
    #[sea_orm(column_type = "Text")]
    pub address: String,
    pub area: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::storage::Entity")]
    Storage,
}

impl Related<super::storage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Storage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
pub struct InsertModel {
    pub house_name: String,
    pub address: String,
    pub area: i32,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct UpdateModel {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    pub house_name: String,
    pub address: String,
    pub area: i32,
}

impl IntoActiveModel<ActiveModel> for UpdateModel {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            house_name: Set(self.house_name),
            address: Set(self.address),
            area: Set(self.area),
        }
    }
}
