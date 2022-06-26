use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("warehouse {0} already exist")]
    WarehouseAlreadyExist(String),
}
