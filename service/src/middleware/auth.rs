use poem::{
    async_trait, http::StatusCode, Endpoint, Error, IntoResponse, Middleware, Request, Response,
    Result,
};

pub struct Authenticate;
pub struct AuthenticateEndpoint<E>(E);

impl<E: Endpoint> Middleware<E> for Authenticate {
    type Output = AuthenticateEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthenticateEndpoint(ep)
    }
}

#[async_trait]
impl<E: Endpoint> Endpoint for AuthenticateEndpoint<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let fail = Err(Error::from_status(StatusCode::UNAUTHORIZED));

        match req.header("X-Access-Token") {
            None => fail,

            Some(token) => match codec::parse(token) {
                None => fail,

                Some(my_claims) => {
                    req.set_data(my_claims.username);

                    // 有 await ，不能使用 Monad 操作
                    self.0.call(req).await.map(IntoResponse::into_response)
                }
            },
        }
    }
}
