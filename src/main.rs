use core::panic;
use dotenv::dotenv;
use std::env;
use std::process::Command;

mod head;
mod base;
mod pr;

use crate::head::head::factory as headFactory;
use crate::base::base::factory as baseFactory;
use crate::pr::pr::factory as prFactory;

fn main() {
    dotenv().ok();

    let repo_path = match env::var("REPO_PATH") {
        Ok(value) => value,
        Err(_) => panic!("Set ENV var REPO_PATH"),
    };

    let head = headFactory(current_head(&repo_path));

    let bases = baseFactory(&head);

    let prs = prFactory(&repo_path, bases, head);

    println!("PRs => {:?}", prs);
   
}

fn current_head(repo_path: &String) -> String {
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

