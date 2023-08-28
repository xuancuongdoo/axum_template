use anyhow::Result;
use serde_json::json;


#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello?name=c").await?.print().await?;

    hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
        "username" : "demo1",
        "pwd" : "welcome"
        }),
    );
    // req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title" : "test"
        }),
    );
    req_create_ticket.await?.print().await?;
    
    hc.do_delete("/api/tickets/5").await?.print().await?;
    hc.do_get("/api/tickets/1").await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;
    Ok(())
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
