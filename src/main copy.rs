use core::panic;
use dotenv::dotenv;
use std::env;
use std::process::Command;

#[derive(Debug)]
#[derive(Clone)]
enum HeadMode {
    Feature,
    BugFix,
    HotFix,
}

#[derive(Debug)]
#[derive(Clone)]
struct Head {
    name: String,
    mode: HeadMode
}

#[derive(Debug)]
#[derive(Clone)]
struct Base {
    name: String,
    labels: Vec<String>,
}

#[derive(Debug)]
struct PR {
    base: Base,
    head: Head,
    url: String,
}

fn main() {
    dotenv().ok();

    let repo_path = match env::var("REPO_PATH") {
        Ok(value) => value,
        Err(_) => panic!("Set ENV var REPO_PATH"),
    };

    let head = get_head(&repo_path);
    let head = Head {
        mode: get_head_type(&head),
        name: head
    };

    let base = get_base(&head);
    let base = base.iter().map(|x| Base{
        name: x.to_string(),
        labels: get_base(&head)
    }).collect::<Vec<_>>();

    let prs = base.iter().map(|x| PR{
        base: x.clone(),
        head: head.clone(),
        url: create_pr(&repo_path, &x, &head),
    }).collect::<Vec<_>>();

    println!("TEMOS => {:?}", prs);
}

fn get_head(repo_path: &String) -> String {
    let output = Command::new("git")
        .current_dir(repo_path)
        .arg("symbolic-ref")
        .arg("HEAD")
        .output();

    let command_response = match output {
        Ok(value) => value,
        Err(err) => panic!("Error at get head {}", err),
    };

    return String::from_utf8(command_response.stdout)
        .unwrap()
        .replace("refs/heads/", "")
        .replace("\n", "");
}

fn get_head_type(head: &String) -> HeadMode {
    let head = head.to_lowercase();
    match head {
        head if head.contains("feature") => HeadMode::Feature,
        head if head.contains("bugfix") => HeadMode::BugFix,
        head if head.contains("hotfix") => HeadMode::HotFix,
        _ => panic!("Your current branch name don't match with gitflow patter")
    }
}

fn get_base(head: &Head) -> Vec<String>{    
    match head.mode {
        HeadMode::Feature => return get_base_env(String::from("FEATURE_TARGETS")),
        HeadMode::BugFix => return get_base_env(String::from("BUGFIX")),
        HeadMode::HotFix => return get_base_env(String::from("HOTFIX_TARGETS")),
    }
}

fn get_base_env(mode: String) -> Vec<String>{
   let response: Vec<String> = Vec::from_iter(env::var(mode).unwrap().split(",").map(String::from));
   return response;
}

fn create_pr(repo_path: &String, base: &Base, head: &Head) -> String {

    let output = Command::new("gh")
        .current_dir(repo_path)
        .arg("pr")
        .arg("create")
        .arg("--title")
        .arg("Test title")
        .arg("--body")
        .arg("The PR body test")
        .arg("--base")
        .arg(&base.name)
        .arg("--head")
        .arg(&head.name)
        .output();

    let command_response = match output {
        Ok(value) => value,
        Err(err) => panic!("Error at create PR {}", err),
    };
    // println!("base => {:?}", &base.name);
    // println!("head => {:?}", &head.name);
    // println!("COMMAND => {:?}", command_response);

    return String::from_utf8(command_response.stdout).unwrap().replace("\n", "");
}
