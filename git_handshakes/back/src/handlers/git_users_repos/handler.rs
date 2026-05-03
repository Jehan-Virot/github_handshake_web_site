use axum::{extract::{Path, State}, Json, http::StatusCode};
use crate::state::AppState;
use crate::handlers::git_users_repos::functions;
use crate::handlers::git_users_repos::structs::{GithubUser, GithubUserRaw, Repo, RepoRaw};


// GET: /search_git/:user
pub async fn search_git_user_handler(
    Path(user): Path<String>,
    State(state): State<AppState>,
) ->  Result<Json<GithubUser>, StatusCode> {
    
    let user_info : GithubUserRaw =  match functions::get_github_user(user, &state).await {
        Ok(user_info) => {user_info},
        Err(err) => {println!("erreur lors de get_github_user: {:?}", err); 
        return Err(StatusCode::INTERNAL_SERVER_ERROR);}
    };
    
    let followers : Vec<GithubUserRaw> = match functions::get_github_followers_or_following(&user_info.followers_url, &state).await {
        Ok(followers) => {followers},
        Err(err) => {println!("erreur lors de search_git_user_handler -> get_github_followerso_or_following (followers): {:?}", err); 
        return Err(StatusCode::INTERNAL_SERVER_ERROR);}
    };

    let following : Vec<GithubUserRaw> = match functions::get_github_followers_or_following(&user_info.following_url, &state).await {
        Ok(following) => {following},
        Err(err) => {println!("erreur lors de search_git_user_handler -> get_github_followerso_or_following (following): {:?}", err); 
        return Err(StatusCode::INTERNAL_SERVER_ERROR);}
    };
    
    let repos : Vec<RepoRaw> = match functions::get_repos_from_user(&user_info.repos_url, &state).await {
        Ok(repos) => {repos},
        Err(err) => {println!("erreur lors de search_git_user_handler -> get_repos_from_user: {:?}", err); 
        return Err(StatusCode::INTERNAL_SERVER_ERROR);}
    };
    
    let complet_user : GithubUser = GithubUser{
        login : user_info.login,
        id : user_info.id,
        avatar_url : user_info.avatar_url,
        followers : followers,
        following : following,
        repos : repos,
        user_type: user_info.user_type,
        name : user_info.name,
    };

    Ok(Json(complet_user))
}