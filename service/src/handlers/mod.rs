pub mod warehouse;
pub mod storage;
pub mod shipping;
pub mod freight_forwarder;

use poem::handler;

#[handler]
pub fn ping() -> &'static str {
    "ice-free-port server"
}
