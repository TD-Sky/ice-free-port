use crate::{reply::Reply, state::State};
use entity::warehouse::{self, Entity as Warehouse};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json, Path},
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, Unchanged};

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
    Json(house): Json<warehouse::Model>,
) -> Reply {
    let house_name = house.house_name.clone();

    let mut act_model: warehouse::ActiveModel = house.into();
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
    Json(house): Json<warehouse::Model>,
) -> Reply {
    let house_name = house.house_name.clone();

    // into 只会得到所有字段皆 Unchanged
    let mut act_model = warehouse::ActiveModel {
        id: Unchanged(house.id),
        house_name: Set(house.house_name),
        address: Set(house.address),
        area: Set(house.area),
    };

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
