use axum::{Router, routing::{get, post, delete, put}};
use crate::handlers::git_users_repos::handler::search_git_user_handler;
use crate::state::AppState;

pub async fn routes() -> Router<AppState>{
    Router::new()
        .route("/search_git_user_info/:user", get(search_git_user_handler))
}