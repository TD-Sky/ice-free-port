use crate::handlers::*;
use poem::Route;

#[allow(unused_imports)]
use poem::{delete, get, post, put};

pub(super) fn router() -> Route {
    Route::new()
          .at("/ping", get(ping))
          .at("/warehouse", get(warehouse::batch)
                           .post(warehouse::insert)
                           .put(warehouse::update))
          .at("/warehouse/:id", delete(warehouse::delete))
          .at("/storage", get(storage::batch)
                         .post(storage::insert)
                         .put(storage::update))
          .at("/storage/:id", delete(storage::delete))
          .at("/freight-forwarder", get(freight_forwarder::batch)
                                   .post(freight_forwarder::insert)
                                   .put(freight_forwarder::update))
          .at("/freight-forwarder/:id", delete(freight_forwarder::delete))
          .at("/shipment", get(shipping::batch)
                          .post(shipping::insert)
                          .put(shipping::update))
          .at("/shipment/:order_num/:nth", delete(shipping::delete))
}
