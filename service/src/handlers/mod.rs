pub mod warehouse;
pub mod storage;
pub mod shipping;
pub mod freight_forwarder;

use poem::handler;

const UNIT_WEIGHT: f64 = 0.05;

#[handler]
pub fn ping() -> &'static str {
    "ice-free-port server"
}
