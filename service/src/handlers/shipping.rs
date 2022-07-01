use entity::{
    prelude::{ShippingItem, ShippingOrder},
    shipping_item, shipping_order,
};

use crate::{models::shipping, reply::Reply, state::State};
use poem::{
    handler,
    web::{Data, Json, Path},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};

#[handler]
pub async fn batch(Data(State { db, .. }): Data<&State>) -> Reply<Vec<shipping::DetailedItem>> {
    let items = ShippingOrder::find()
        .left_join(ShippingItem)
        .select_only()
        .column(shipping_item::Column::OrderNum)
        .column(shipping_order::Column::CompanyId)
        .column(shipping_item::Column::Nth)
        .column(shipping_item::Column::ShipmentDate)
        .column(shipping_item::Column::Quantity)
        .column(shipping_item::Column::Ton)
        .order_by_asc(shipping_item::Column::ShipmentDate)
        .order_by_desc(shipping_item::Column::Quantity)
        .into_model::<shipping::DetailedItem>()
        .all(db)
        .await
        .expect("[Error] handlers::shipping::batch");

    Reply::builder().data(items).finish()
}

#[handler]
pub async fn insert(
    Data(State { db, id_gen }): Data<&State>,
    Json(shipping_order): Json<shipping::Order>,
) -> Reply {
    let order_num = id_gen.lock().unwrap().get_id();

    let act_shipping_order = shipping_order::ActiveModel {
        num: Set(order_num),
        company_id: Set(shipping_order.company_id),
    };

    ShippingOrder::insert(act_shipping_order)
        .exec(db)
        .await
        .expect("[Error] handlers::shipping::insert");

    let act_shipping_items: Vec<_> = shipping_order
        .items
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let mut act_model = item.into_active_model();
            act_model.order_num = Set(order_num);
            act_model.nth = Set((idx as i16) + 1);
            act_model
        })
        .collect();

    ShippingItem::insert_many(act_shipping_items)
        .exec(db)
        .await
        .expect("[Error] handlers::shipping::insert");

    Reply::default()
}

#[handler]
pub async fn update(
    Data(State { db, .. }): Data<&State>,
    Json(shipping_item): Json<shipping_item::UpdateModel>,
) -> Reply {
    let mut act_model = shipping_item.into_active_model();

    act_model
        .update(db)
        .await
        .expect("[Error] handlers::shipping::update");

    Reply::default()
}

#[handler]
pub async fn delete(Data(State { db, .. }): Data<&State>, Path(id): Path<(i64, i16)>) -> Reply {
    let order_num = id.0;

    ShippingItem::delete_by_id(id)
        .exec(db)
        .await
        .expect("[Error] handlers::shipping::delete");

    let item_count = ShippingItem::find()
        .filter(shipping_item::Column::OrderNum.eq(order_num))
        .count(db)
        .await
        .expect("[Error] handlers::shipping::delete");

    if item_count == 0 {
        ShippingOrder::delete_by_id(order_num)
            .exec(db)
            .await
            .expect("[Error] handlers::shipping::delete");
    }

    Reply::default()
}
