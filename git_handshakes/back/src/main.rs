use axum::{Router, routing::get, extract::{State}};
use std::net::SocketAddr;
use reqwest::Client;
use tower_http::cors::CorsLayer;

mod state;
mod handlers;
mod routes;
use crate::state::AppState;
use crate::handlers::git_users_repos::handler::search_git_user_handler;



#[tokio::main]
async fn main() {

    let cors = CorsLayer::permissive(); //pour permettre l'accès au container front end 

    let client = Client::new();
    let state = AppState {client: client.clone()}; //créer un state pour le keep alive pour les demandes régulières à l'api github (perdre moins de temps)

    let app = Router::new()
        .route("/search_git/:user", get(search_git_user_handler))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0,0,0,0], 3000)); 

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();    
}

