// use async_std::prelude::*;
use tide::{prelude::*, Request, Response};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    let file = include_str!("../../index.html");
    app.at("/").get(move |req| async move {
        dbg!(req);
        Response::new(200)
            .body_string((&file).to_string())
            .append_header("content-type", "text/html;charset=utf-8")
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
