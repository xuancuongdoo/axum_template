use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
//
//e
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    LoginFail,
    AuthFaultNoAuthTokenCookie,
    TicketDeleteFailIdNotFound { id: u64 },
    TicketGetFailIdNotFound { id: u64 },
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->>  {:<12} - {self:?}", "INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED").into_response()
    }
}
