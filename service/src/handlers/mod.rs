pub mod freight_forwarder;
pub mod shipping;
pub mod storage;
pub mod user;
pub mod warehouse;

use poem::handler;

#[handler]
pub fn ping() -> &'static str {
    "ice-free-port server"
}
