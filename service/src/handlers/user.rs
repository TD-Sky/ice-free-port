use entity::{
    prelude::Registry,
    registry::{self, User},
};

use crate::{errors::Code, reply::Reply, state::State};
use poem::{
    handler,
    web::{Data, Json},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[handler]
pub async fn login(Data(State { db, .. }): Data<&State>, Json(user): Json<User>) -> Reply<String> {
    let reg_user = Registry::find()
        .filter(registry::Column::Username.eq(user.name.clone()))
        .one(db)
        .await
        .expect("[ERROR] handlers::user::login");

    match reg_user {
        None => Reply::builder()
            .code(Code::UserNotFound)
            .msg(user.name)
            .finish(),

        Some(reg_user) => match user.pass(&reg_user.pswd) {
            true => Reply::builder()
                .data(codec::gen(reg_user.id, reg_user.username))
                .finish(),

            false => Reply::builder()
                .code(Code::IncorrectPassword)
                .msg(user.name)
                .finish(),
        },
    }
}
