use async_trait::async_trait;
use axum::{http::{Request, request::Parts}, middleware::Next, response::Response, extract::FromRequestParts, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::{web::AUTH_TOKEN, Error, Result, ctx::Ctx};

pub async fn mw_require_auth<B>(
    ctx: Request<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection =  Error;
    
    async fn from_request_parts(parts : &mut Parts , _state: &S) -> Result<Self> {
        println!("->  {:<12} - Ctx" , "Extractor");
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFaultNoAuthTokenCookie)
            .and_then(parse_token)?;
       Ok(Ctx::new(user_id)) 
    }
}


fn parse_token(token : String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)$"#,
        &token
    ).ok_or(Error::AuthFaultNoAuthTokenCookie)?;

    let user_id : u64 = user_id.parse().map_err(|_| Error::AuthFaultNoAuthTokenCookie)?;
    Ok((user_id, exp.to_string(), sign.to_string()))

}
