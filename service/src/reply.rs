use crate::errors::Code;
use poem::{http::StatusCode, IntoResponse, Response, ResponseBuilder};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::marker::PhantomData;

#[skip_serializing_none]
#[derive(Serialize)]
struct Body<T = ()> {
    code: u16,
    msg: Option<String>,
    data: Option<T>,
}

pub struct Reply<T = ()> {
    response: Response,
    marker: PhantomData<T>,
}

pub struct ReplyBuilder<T> {
    body: Body<T>,
    response: ResponseBuilder,
}

// 只提供 T = () 时的默认值
impl Default for Reply {
    fn default() -> Self {
        Self::builder().finish()
    }
}

impl<T: Serialize + Send> IntoResponse for Reply<T> {
    fn into_response(self) -> Response {
        self.response
    }
}

impl<T> Reply<T> {
    pub fn builder() -> ReplyBuilder<T> {
        ReplyBuilder {
            body: Body {
                code: Code::OK as u16,
                msg: None,
                data: None,
            },

            response: Response::builder().content_type("application/json; charset=utf-8"),
        }
    }
}

impl<T: Send + Serialize> ReplyBuilder<T> {
    pub fn code(mut self, code: Code) -> Self {
        self.body.code = code as u16;

        self
    }

    pub fn msg(mut self, msg: String) -> Self {
        self.body.msg = Some(msg);

        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.body.data = Some(data);

        self
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.response = self.response.status(status);

        self
    }

    pub fn finish(self) -> Reply<T> {
        Reply {
            response: match serde_json::to_vec(&self.body) {
                Ok(bs) => self.response.body(bs),

                Err(e) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json; charset=utf-8")
                    .body(
                        serde_json::to_vec(&Body {
                            code: Code::FailedToDeserialize as u16,
                            msg: Some(e.to_string()),
                            data: None as Option<T>,
                        })
                        .unwrap(),
                    ),
            },

            marker: PhantomData,
        }
    }
}
