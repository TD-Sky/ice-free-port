#[cfg(test)]
mod tests;

use jwt_simple::prelude::*;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref KEY: HS256Key =
        HS256Key::from_bytes(env::var("SECRET").expect("lose SECRET").as_bytes());
}

#[derive(Serialize, Deserialize)]
pub struct MyClaims {
    pub user_id: i64,
    pub username: String,
}

pub fn gen(user_id: i64, username: String) -> String {
    let claims = Claims::with_custom_claims(MyClaims { user_id, username }, Duration::from_days(3))
        .with_issuer("ice-free-port");

    KEY.authenticate(claims)
        .expect("[ERROR] Failed to generate token")
}

pub fn parse(token: &str) -> Option<MyClaims> {
    KEY.verify_token::<MyClaims>(token, None)
        .ok()
        .map(|claims| claims.custom)
}
