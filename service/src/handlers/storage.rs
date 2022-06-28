use entity::storage::{self, Entity as Storage};

use super::UNIT_WEIGHT;
use crate::{reply::Reply, state::State};
use poem::{
    handler,
    web::{Data, Json, Path},
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, EntityTrait, QueryOrder, Set, Unchanged};

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
    Json(stock): Json<storage::Model>,
) -> Reply {
    let ton = UNIT_WEIGHT * (stock.quantity as f64);

    let mut act_model: storage::ActiveModel = stock.into();
    act_model.id = Set(id_gen.lock().unwrap().get_id());
    act_model.ton = Set(ton);
    act_model.duration = NotSet;

    Storage::insert(act_model)
        .exec(db)
        .await
        .expect("[Error] handlers::storage::insert");

    Reply::default()
}

#[handler]
pub async fn update(
    Data(State { db, .. }): Data<&State>,
    Json(stock): Json<storage::Model>,
) -> Reply {
    let mut act_model = storage::ActiveModel {
        id: Unchanged(stock.id),
        warehouse_id: NotSet,
        store_date: Set(stock.store_date),
        license_plate_number: Set(stock.license_plate_number),
        quantity: Set(stock.quantity),
        ton: Set(UNIT_WEIGHT * (stock.quantity as f64)),
        duration: NotSet,
    };

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
