use axum::{http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{web::AUTH_TOKEN, Error, Result};

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("-> {:<12} -  mw_require_auth ", "HANDLER");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let (user_id, exp, sign) = auth_token.ok_or(
        Error::AuthFaultNoAuthTokenCookie
    ).and_then(
            parse_token
        )?;
    Ok(next.run(req).await)
}

fn parse_token(token : String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!{(
        r#"^user-(\d+)\.(.+)\.(.+)$"#,
        &token
    )}.ok_or(Error::AuthFaultNoAuthTokenCookie)?;

    let user_id : u64 = user_id.parse()?;
    Ok((user_id, exp, sign))

    todo!()
}
