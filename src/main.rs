use std::path::PathBuf;
use axum::{
    Router,
    routing::get,
};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use utoipa_swagger_ui::SwaggerUi;
use tower_http::services::ServeDir;

//Sqlx write sql query in the rust code
/*
#[derive(Clone)]
struct MyState {
    pool: PgPool,
}
*/

/*
async fn retrieve(
    Path(id): Path<i32>,
    State(state): State<MyState>
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()))
    }
}
*/

/*
async fn add(
    State(state): State<MyState>,
    Json(data): Json<TodoNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (note) VALUES ($1) RETURNING id, note"
    ).bind(&data.note)
        .fetch_one(&state.pool)
        .await {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()))
    }
}

*/

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn greet_world() -> &'static str {"Hey!"}
async fn todos() -> &'static str {"What can I do for you?"}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum{

//async fn main( #[shuttle_shared_db::Postgres] pool: PgPool)
//    -> shuttle_axum::ShuttleAxum {
    //sqlx::migrate!("./migrations")
    //    .run(&pool)
    //    .await
    //    .map_err(CustomError::new)?;

    //sqlx::migrate!().run(&pool)
    //    .await
    //    .map_err(CustomError::new)?;
    //let state = MyState{ pool };
    #[derive(OpenApi)]
    struct ApiDoc;
    let router = Router::new().
        route("/", get(hello_world))
        .route("/greetings",get(greet_world))
        .route("/get_todo",get(todos))
        .merge(SwaggerUi::new("/swagger-ui").url("api-doc/openapi.json",ApiDoc::OpenAPI()))
        .nest_service("/assets",ServeDir::new(PathBuf::from("assets")));
        //.route("/todos",post(add))
        //.route("/todos/:id",get(retrieve))
        //.with_state(state);
    Ok(router.into())
}
/*
#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
}

#[derive(Serialize, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
}

*/


