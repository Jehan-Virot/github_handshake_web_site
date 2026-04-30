use axum::{Router, routing::{get,post, put, delete}, extract::{Path, Json, State}};
use std::net::SocketAddr;
use reqwest::Client;
use serde::Deserialize

#[derive[Clone]]
struct AppState{
    client:Client,
}

#[derive(Deserialize)]
struct GithubUser{
    login : String,
    id : u64,
    followers_url : String,
    following_url : String,
    repos_url : String,
    #[serde(rename = "type")]
    user_type: String,
    name : String,
}

#[tokio::main]
async fn main() {

    let client = Client::new();
    let state = AppState {client};

    let app = Router::new()
        .route("/search_git/:user", get(search_git_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0,0,0,0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();    
}

// GET: /search_git/:user
async fn search_git_handler(
    Path(user): Path<String>,
    State(state): State<AppState>,
) ->  Result<Json<GithubUser>, StatusCode> {
    
    let url = format!("https://api.github.com/users/{}", user)
    let res = state.client
        .get(url)
        .header("User-Agent", "axum-app")
        .send()
        .await?
        .json()
        .await?;
    
}