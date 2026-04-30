use serde::Deserialize

#[derive[Deserialize]] //pour le json
pub(in crate::handlers::git_users) struct GithubUserRaw{
    login : String,
    id : u64,
    followers_url : String,
    following_url : String,
    repos_url : String,
    #[serde(rename = "type")]
    user_type: String,
    name : String,
}

#[derive[Deserialize]]
pub(in crate::handlers::git_users) struct GithubUser{
    login : String,
    id : u64,
    followers : Vec<String>,
    following : Vec<String>,
    repos : Vec<String>,
    #[serde(rename = "type")]
    user_type: String,
    name : String,
}