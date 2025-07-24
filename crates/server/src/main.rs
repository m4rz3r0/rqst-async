use miniserve::{http, Content, Request, Response};
use serde_json::Value;

async fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

async fn chat(req: Request) -> Response {
    if let Request::Post(data) = req {
        let mut json_data: Value = match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(_) => return Err(http::StatusCode::from_u16(400).unwrap())
        };

        if let Value::Array(messages) = &mut json_data["messages"] {
            messages.push(Value::String(String::from("Fixed response")));

            println!("{json_data}");

            Ok(Content::Json(json_data.to_string()))
        } else {
            Err(http::StatusCode::from_u16(400).unwrap())
        }
    } else {
        Err(http::StatusCode::from_u16(400).unwrap())
    }
}

#[tokio::main]
async fn main() {
    miniserve::Server::new()
    .route("/", index)
    .route("/chat", chat)
    .run()
    .await
}
