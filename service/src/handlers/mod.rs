pub mod warehouse;
pub mod storage;

use poem::handler;

const UNIT_WEIGHT: f64 = 0.05;

#[handler]
pub fn ping() -> &'static str {
    "from service"
}
