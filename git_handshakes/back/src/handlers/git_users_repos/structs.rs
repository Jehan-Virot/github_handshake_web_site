use serde::{Deserialize, Serialize};


//Structs des user githubs 
//Raw => Avant traitement, il y a des urls qui mènent vers une autre api avec l'information
//sans Raw => Après avoir cherché les infos manquants via les urls
#[derive(Deserialize, Serialize, Debug, Clone)] //permet de passer d'un json à cet objet
pub(in crate) struct GithubUserRaw{
    pub login : String,
    pub id : u64,
    pub avatar_url : String,
    pub followers_url : String,
    pub following_url : String,
    pub repos_url : String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub name : String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]//permet de passer de cet objet à un json
pub(in crate) struct GithubUser{
    pub login : String,
    pub id : u64,
    pub avatar_url : String,
    pub followers : Vec<GithubUserRaw>,
    pub following : Vec<GithubUserRaw>,
    pub repos : Vec<RepoRaw>,
    #[serde(rename = "type")]
    pub user_type: String,
    pub name : String,
}


//structs pour les repos
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(in crate) struct RepoRaw{
    pub name : String,
    pub full_name : String, 
    pub description : String,
    pub url : String,
    pub contributors_url : String,
    pub languages_url : String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(in crate) struct Repo{
    pub name : String,
    pub full_name : String, 
    pub description : String,
    pub url : String,
    pub contributors_url : Vec<GithubUserRaw>,
    pub languages_url : Vec<(String, i64)>,
}