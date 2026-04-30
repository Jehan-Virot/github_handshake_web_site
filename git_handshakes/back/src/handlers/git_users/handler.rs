use axum::{extract::{Path, Json}}
use crate::state::AppState

// GET: /search_git/:user
async pub fn search_git_handler(
    Path(user): Path<String>,
    State(state): State<AppState>,
) ->  Result<Json<GithubUser>, StatusCode> {
    
    let url = format!("https://api.github.com/users/{}", user)
    let user = state.client
        .get(url)
        .header("User-Agent", "axum-app")
        .send()
        .await?
        .json()
        .await?
    
}