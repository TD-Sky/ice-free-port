use entity::{prelude::Storage, storage};

use crate::{reply::Reply, state::State};
use poem::{
    handler,
    web::{Data, Json, Path},
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, QueryOrder, Set};

#[handler]
pub async fn batch(Data(State { db, .. }): Data<&State>) -> Reply<Vec<storage::Model>> {
    let stocks = Storage::find()
        .order_by_asc(storage::Column::StoreDate)
        .order_by_desc(storage::Column::Quantity)
        .all(db)
        .await
        .expect("[Error] handlers::storage::batch");

    Reply::builder().data(stocks).finish()
}

#[handler]
pub async fn insert(
    Data(State { db, id_gen }): Data<&State>,
    Json(stock): Json<storage::InsertModel>,
) -> Reply {
    let mut act_model = stock.into_active_model();
    act_model.id = Set(id_gen.lock().unwrap().get_id());

    Storage::insert(act_model)
        .exec(db)
        .await
        .expect("[Error] handlers::storage::insert");

    Reply::default()
}

#[handler]
pub async fn update(
    Data(State { db, .. }): Data<&State>,
    Json(stock): Json<storage::UpdateModel>,
) -> Reply {
    let mut act_model = stock.into_active_model();

    act_model
        .update(db)
        .await
        .expect("[Error] handlers::storage::update");

    Reply::default()
}

#[handler]
pub async fn delete(Data(State { db, .. }): Data<&State>, Path(id): Path<i64>) -> Reply {
    Storage::delete_by_id(id)
        .exec(db)
        .await
        .expect("[Error] handlers::storage::delete");

    Reply::default()
}
