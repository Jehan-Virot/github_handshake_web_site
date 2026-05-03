use crate::state::AppState;
use crate::handlers::git_users_repos::structs::{GithubUserRaw, GithubUser, RepoRaw, Repo};
use axum::{Json, routing::get};

//functions for handler get users
pub(in crate::handlers::git_users_repos) async fn get_github_user(username:String, state:&AppState) -> Result<GithubUserRaw, reqwest::Error> {
    
    let url = format!("https://api.github.com/users/{}", username);
    
    let user : GithubUserRaw = state.client.get(url).header("User-Agent", "axum-app").send()
        .await?
        .json()
        .await?;

    Ok(user)
}
pub(in crate::handlers::git_users_repos) async fn get_github_followers_or_following(url:&str, state:&AppState) -> Result<Vec<GithubUserRaw>, reqwest::Error> {
    let users : Vec<GithubUserRaw> = state.client.get(url).header("User-Agent", "axum-app").send()
        .await?
        .json()
        .await?;
    
    Ok(users)
}
pub(in crate::handlers::git_users_repos) async fn get_repos_from_user(url:&str, state:&AppState) -> Result<Vec<RepoRaw>, reqwest::Error> {
    let repos : Vec<RepoRaw> = state.client.get(url).header("User-Agent", "axum-app").send()
        .await?
        .json()
        .await?;

    Ok(repos)
}
