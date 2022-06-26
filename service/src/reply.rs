use crate::error::MyError;
use poem::{
    http::{header::CONTENT_TYPE, StatusCode},
    web::IntoResponse,
    Response,
};
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Reply<T = ()> {
    code: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

pub struct ReplyBuilder<T = ()> {
    code: u16,
    msg: Option<String>,
    data: Option<T>,
}

impl<T: Serialize + Send> IntoResponse for Reply<T> {
    fn into_response(self) -> Response {
        let data = match serde_json::to_vec(&self) {
            Ok(data) => data,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string())
            }
        };

        Response::builder()
            .header(CONTENT_TYPE, "application/json; charset=utf-8")
            .body(data)
    }
}

impl<T: Serialize> From<Result<T, MyError>> for Reply<T> {
    fn from(res: Result<T, MyError>) -> Self {
        match res {
            Ok(data) => Reply {
                code: 0,
                msg: None,
                data: Some(data),
            },

            Err(err) => match err {
                MyError::WarehouseAlreadyExist(name) => Reply {
                    code: 1,
                    msg: Some(name),
                    data: None,
                },
            },
        }
    }
}

impl<T> Reply<T> {
    pub fn builder() -> ReplyBuilder<T> {
        ReplyBuilder {
            code: 0,
            msg: None,
            data: None,
        }
    }
}

impl<T> ReplyBuilder<T> {
    pub fn code(mut self, code: u16) -> Self {
        self.code = code;

        self
    }

    pub fn msg(mut self, msg: String) -> Self {
        self.msg = Some(msg);

        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);

        self
    }

    pub fn build(self) -> Reply<T> {
        Reply {
            code: self.code,
            msg: self.msg,
            data: self.data,
        }
    }
}
