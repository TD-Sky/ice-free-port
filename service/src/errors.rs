#[derive(Debug)]
pub enum Code {
    OK,
    FailedToDeserialize,
    UserNotFound,
    IncorrectPassword,
    WarehouseAlreadyExist,
    CompanyAlreadyExist,
}
