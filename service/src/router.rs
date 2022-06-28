use crate::handlers::*;
use poem::Route;

#[allow(unused_imports)]
use poem::{delete, get, post, put};

pub(super) fn router() -> Route {
    Route::new()
          .at("/ping", get(ping))
          .nest("/warehouse",
                Route::new()
                      .at("/", get(warehouse::batch)
                              .post(warehouse::insert)
                              .put(warehouse::update)
                       )
                      .at("/:id", delete(warehouse::delete))
           )
          .nest("/storage",
                Route::new()
                       .at("/", get(storage::batch)
                               .post(storage::insert)
                               .put(storage::update)
                        )
                    .at("/:id", delete(storage::delete))
           )
}
