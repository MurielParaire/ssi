use rand::Rng;
use once_cell::sync::Lazy;
use axum::{routing::get, Router, Json, serve, http::StatusCode, response::{IntoResponse, Response}};
use anyhow::Result;

// verb list
static VERBS: Lazy<Vec<&str>> = Lazy::new(|| vec![
    "smiling",
    "thinking",
    "running",
    "climbing",
    "chasing",
    "sleeping",
    "riding",
    "fighting",
    "jumping",
    "chilling",
    "crying",
    "working",
    "playing",
    "screaming"
]);

// create custom struct and implement IntoResponse to be able to use this Error with axum
pub struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error occured :( : {}", self.0),
        )
            .into_response()
    }
}


// function to return one verb
pub async fn get_verb() -> Result<Json<String>, ApiError> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..VERBS.len());
    Ok(Json(format!("{}", VERBS[index])))
}



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_verb));

    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    // run app
    serve(listener, app).await.unwrap();
}
