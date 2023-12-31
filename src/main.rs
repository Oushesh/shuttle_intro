use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}
async fn greet() -> &'static str{
    "Greetings!"
}
async fn goodbye() -> &'static str{
    "Goodbye!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world))
        .route("/greet",get(greet))
        .route("/goodbye",get(goodbye));
    Ok(router.into())
}

