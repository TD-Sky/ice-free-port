//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use lazy_static::lazy_static;
use sea_orm::entity::prelude::*;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::env;

lazy_static! {
    static ref SALT: Vec<u8> = env::var("SALT").expect("lose SALT").into();
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "registry")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub pswd: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    password: String,
}

impl User {
    pub fn pass(&self, correct_pswd: &str) -> bool {
        format!(
            "{:x}",
            Sha256::new_with_prefix(&self.password)
                .chain_update(SALT.as_slice())
                .finalize()
        ) == correct_pswd
    }
}
