use octocrab::{Octocrab};
use std::env;
use octocrab::Error as OctocrabError;
use std::process::Command;
use std::str;
use dotenv::dotenv;
use std::error::Error;
use octocrab::models::pulls::PullRequest;
use std::collections::HashMap;

#[derive(Debug)]
enum Branch {
    BUGFIX,
    HOTFIX,
    FEATURE
}


struct GitHubUser {
    id: String,
    name: String,
}

struct GitHubRepo {
    url: String,
    name: String
}

struct GitHubPullRequest {
    id: String,
    name: String,
    from: String,
    to: String
}


// # TODO
// Get current repo

#[tokio::main]
async fn main() -> Result<(), OctocrabError> {
    dotenv().ok();

    let octocrab = instance_octocrab();
    let owner = get_current_user(&octocrab).await;    
    let repo = get_repo_name();

    let pr = create_pr(&octocrab, &owner, &repo).await;
    request_review(&owner, &repo, &pr).await;

    Ok(())
}

fn instance_octocrab() -> Octocrab {
    let gh = env::var("GIT_TOKEN").expect("$GIT_TOKEN is not set");
    return Octocrab::builder().personal_token(gh).build().unwrap();
}

async fn get_current_user(octocrab: &Octocrab) -> GitHubUser{
    let current_user = octocrab.current()
    .user()
    .await;

    match current_user {
        Ok(user) => return GitHubUser {
            id: user.id.to_string(),
            name: user.login
        },
        Err(message) => panic!("Error at get user {}", message)
    }
}

fn get_repo_name() -> GitHubRepo{
    let repo_path = match env::var("REPO_PATH") {
        Ok(value) => value,
        Err(_) => String::from("./")

    };

    let output = Command::new("git")
        .current_dir(repo_path)
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output().unwrap();

    let url = String::from_utf8(output.stdout).unwrap();   
   
    return GitHubRepo {
        name: between(&url, "/", ".git"),
        url: url
    };  
}

fn between(line: &String, start: &str, end: &str) -> String {
    let start_bytes = line.find(start).unwrap_or(0); 
    let end_bytes = line.find(end).unwrap_or(line.len()); 
    return String::from(&line[start_bytes+1..end_bytes]);
}

async fn create_pr(octocrab: &Octocrab, owner: &GitHubUser, repo: &GitHubRepo) -> PullRequest{
    println!("owner {}", owner.name);
    println!("repo {}", repo.name);
    
    let pr =  octocrab 
    .pulls(&owner.name, &repo.name)
    .create("title", "gusttavodev-patch-1", "master")
    .body("hello world!")
    .send().await;

    match pr {
        Ok(data) =>  return data,
        Err(message) => panic!("TIVEMOS =:  {}", message)
    };
}

async fn request_review(owner: &GitHubUser, repo: &GitHubRepo, pr: &PullRequest) -> Result<(), Box<dyn Error>>{
    let mut params = HashMap::new();
    params.insert("reviewers", ("PoorlyDefinedBehaviour", "AA"));
    
    // https://docs.github.com/en/rest/pulls/review-requests#request-reviewers-for-a-pull-request
    let gh = env::var("GIT_TOKEN").expect("$GIT_TOKEN is not set");
    let route = format!("https://api.github.com/repos/
    {}/{}/pulls/{}/requested_reviewers
    ", owner.name, repo.name, pr.id);
    
    let client = reqwest::Client::new();

    let resp = client.post(route)
        .header("Authorization", format!("Bearer {}", gh))
        .json(&params)
        .send()
        .await?
        .json()
        .await?;

    println!("RESPONSE => {:#?}", resp);

    Ok(())
}
