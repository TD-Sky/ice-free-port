use crate::{handlers::*, middleware::auth::Authenticate};
use poem::{Route, Endpoint, EndpointExt};

#[allow(unused_imports)]
use poem::{delete, get, post, put};

pub(super) fn router() -> Route {
    Route::new()
        .at("/ping", get(ping))
        .at("/login", post(user::login))
        .nest_no_strip("/",  privileged())
}

fn privileged() -> impl Endpoint {
    Route::new()
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
          .with(Authenticate)
}
