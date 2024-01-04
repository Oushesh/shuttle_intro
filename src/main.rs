use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn greet_world() -> &'static str {"Hey!"}
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().
        route("/", get(hello_world))
        .route("/greetings",get(greet_world));
    Ok(router.into())
}
