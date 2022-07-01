use entity::{freight_forwarder, prelude::FreightForwarder};

use crate::{reply::Reply, state::State};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json, Path},
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

#[handler]
pub async fn batch(Data(State { db, .. }): Data<&State>) -> Reply<Vec<freight_forwarder::Model>> {
    let companies = FreightForwarder::find()
        .all(db)
        .await
        .expect("[Error] handlers::freight_forwarder::batch");

    Reply::builder().data(companies).finish()
}

#[handler]
pub async fn insert(
    Data(State { db, id_gen }): Data<&State>,
    Json(company): Json<freight_forwarder::InsertModel>,
) -> Reply {
    let company_name = company.company_name.clone();

    let mut act_model = company.into_active_model();
    act_model.id = Set(id_gen.lock().unwrap().get_id());

    match FreightForwarder::insert(act_model).exec(db).await {
        Ok(_) => Reply::default(),

        Err(_) => Reply::builder()
            .code(3)
            .msg(company_name)
            .status(StatusCode::CONFLICT)
            .finish(),
    }
}

#[handler]
pub async fn update(
    Data(State { db, .. }): Data<&State>,
    Json(company): Json<freight_forwarder::UpdateModel>,
) -> Reply {
    let company_name = company.company_name.clone();

    let mut act_model = company.into_active_model();

    match act_model.update(db).await {
        Ok(_) => Reply::default(),

        Err(_) => Reply::builder()
            .code(3)
            .msg(company_name)
            .status(StatusCode::CONFLICT)
            .finish(),
    }
}

#[handler]
pub async fn delete(Data(State { db, .. }): Data<&State>, Path(id): Path<i64>) -> Reply {
    FreightForwarder::delete_by_id(id)
        .exec(db)
        .await
        .expect("[Error] handlers::freight_forwarder::delete");

    Reply::default()
}
