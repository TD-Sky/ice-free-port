use entity::{prelude::Warehouse, warehouse};

use crate::{reply::Reply, state::State};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json, Path},
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

#[handler]
pub async fn batch(Data(State { db, .. }): Data<&State>) -> Reply<Vec<warehouse::Model>> {
    let houses = Warehouse::find()
        .all(db)
        .await
        .expect("[Error] handlers::warehouse::batch");

    Reply::builder().data(houses).finish()
}

#[handler]
pub async fn insert(
    Data(State { db, id_gen }): Data<&State>,
    Json(house): Json<warehouse::InsertModel>,
) -> Reply {
    let house_name = house.house_name.clone();

    let mut act_model = house.into_active_model();
    act_model.id = Set(id_gen.lock().unwrap().get_id());

    match Warehouse::insert(act_model).exec(db).await {
        Ok(_) => Reply::default(),

        Err(_) => Reply::builder()
            .code(2)
            .msg(house_name)
            .status(StatusCode::CONFLICT)
            .finish(),
    }
}

#[handler]
pub async fn update(
    Data(State { db, .. }): Data<&State>,
    Json(house): Json<warehouse::UpdateModel>,
) -> Reply {
    let house_name = house.house_name.clone();

    let mut act_model = house.into_active_model();

    match act_model.update(db).await {
        Ok(_) => Reply::default(),

        Err(_) => Reply::builder()
            .code(2)
            .msg(house_name)
            .status(StatusCode::CONFLICT)
            .finish(),
    }
}

#[handler]
pub async fn delete(Data(State { db, .. }): Data<&State>, Path(id): Path<i64>) -> Reply {
    Warehouse::delete_by_id(id)
        .exec(db)
        .await
        .expect("[Error] handlers::warehouse::delete");

    Reply::default()
}
