use octocrab::{Octocrab};
use std::env;
use octocrab::Error;
use std::process::Command;
use std::str;

#[derive(Debug)]
struct GitHubUser {
    id: String,
    name: String,
}

// # TODO
// Get current repo

#[tokio::main]
async fn main() -> Result<(), Error> {
    let octocrab = instance_octocrab();
    let owner = get_current_user(&octocrab).await;
    
    let repo = get_repo_name();

    println!("Current User => {:?}", owner);
    
    // Use Tokio
    // create_pr(&octocrab, owner, repo).await;

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

fn get_repo_name(){
    let output = Command::new("git")
    .arg("config")
    .arg("--get")
    .arg("remote.origin.url")
    .output().unwrap();
    let response =  String::from_utf8(output.stdout).unwrap();
    println!("AA => {}", response);
}
async fn create_pr(octocrab: &Octocrab, owner: &str, repo: &str) {
    let pr =  octocrab 
    .pulls(owner, repo)
    .create("title", "gusttavodev-patch-1", "master")
    .body("hello world!")
    .send().await;

    match pr {
        Ok(data) => println!("{:?}", data),
        Err(message) => panic!("Error at creating PR {}", message)
    }
}

