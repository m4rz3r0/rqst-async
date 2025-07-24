use miniserve::{http, Content, Request, Response};
use serde_json::Value;
use tokio::join;

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
            let messages_vec = messages.iter().flat_map(|message| if let Value::String(msg) = message {
                Some(msg.clone())
            } else {
                None
            }).collect::<Vec<String>>();

            let responses = tokio::spawn(async move { chatbot::query_chat(&messages_vec).await });
            let random_number = tokio::spawn(chatbot::gen_random_number());

            let (responses,random_number) = join!(responses, random_number);
            let Ok(responses) = responses else {
                return Err(http::StatusCode::from_u16(400).unwrap());
            };
            let Ok(random_number) = random_number else {
                return Err(http::StatusCode::from_u16(400).unwrap());
            };
            messages.push(Value::String(responses[random_number % responses.len()].clone()));
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
