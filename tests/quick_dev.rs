use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8000")?;

    hc.do_get("/hello?name=yogesh").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username":"demo1",
            "pwd":"welcome"
        }),
    );

    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_get("/hello2/shahi").await?.print().await?;

    // hc.do_delete("/api/tickets/1").await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
