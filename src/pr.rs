
pub mod pr {
    use core::panic;
    use crate::head::head::Head;
    use crate::base::base::Base;
    use std::process::Command;
    
    #[derive(Debug)]
    pub struct Pr {
        base: Base,
        head: Head,
        url: String,
    }    

    pub fn factory(repo_path: &String, base: Vec<Base>, head: Head)-> Vec<Pr>{       
        return base.iter().map(|x| Pr{
            base: x.clone(),
            head: head.clone(),
            url: create_pr(&repo_path, &x, &head),
        }).collect::<Vec<_>>();    
    }

    fn create_pr(repo_path: &String, base: &Base, head: &Head) -> String {

        println!("!Head {}",head.name );
        println!("!Base {}",base.name );
        println!("!repo_path {}",repo_path );
        
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
    
        return String::from_utf8(command_response.stdout).unwrap().replace("\n", "");
    }
    
}

