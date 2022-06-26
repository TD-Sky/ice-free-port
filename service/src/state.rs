use sea_orm::DatabaseConnection;
use snowflake::SnowflakeIdBucket;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct State {
    pub db: DatabaseConnection,
    pub id_gen: Arc<Mutex<SnowflakeIdBucket>>,
}

impl State {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            id_gen: Arc::new(Mutex::new(SnowflakeIdBucket::new(114, 514))),
        }
    }
}
