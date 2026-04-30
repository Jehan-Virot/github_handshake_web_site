use axum::{Router, routing::{get,post, put, delete}, extract::{Path, Json, State}};
use std::net::SocketAddr;
use reqwest::Client;
use crate::state::AppState
use tower_http::cors::CorsLayer;


#[tokio::main]
async fn main() {

    let cors = CorsLayer::permissive(); //pour permettre l'accès au container front end 

    let client = Client::new();
    let pub state = AppState {client}; //créer un state pour le keep alive pour les demandes régulières à l'api github (perdre moins de temps)

    let app = Router::new()
        .route("/search_git/:user", get(search_git_handler))
        .with_state(state);
        .layer(cors)

    let addr = SocketAddr::from(([0,0,0,0], 3000)); 

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();    
}

