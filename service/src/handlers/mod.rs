pub mod warehouse;

use poem::handler;

#[handler]
pub fn ping() -> &'static str {
    "from service"
}
