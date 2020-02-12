use async_std::prelude::*;
use tide::{prelude::*, Request, Response};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    let file = include_str!("../index.html");
    app.at("/").get(move |_| async move {
        // Response::new(200)
        //     .body_string((&file).to_string())
        //     .set_mime(mime::TEXT_HTML_UTF_8)
        //     .append_header("content-type", " text/html;charset=utf-8")
        "hello world"
    });
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
