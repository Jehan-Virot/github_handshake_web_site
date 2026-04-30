use axum::{Router, routing::{get, post, delete, put}}
use crate::handlers::git_users::search_git_handler
use crate::state::AppState

async pub fn routes() -> Router<AppState>{
    let router = Router::new()
        .route("/search_git_user_info/:user", get(search_git_handler))
}