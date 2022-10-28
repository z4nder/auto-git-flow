
pub mod base {
    use core::panic;
    use std::env;
    use crate::head::head::Head;

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Base {
        pub name: String,
        pub labels: Vec<String>,
    }    

    pub fn factory(head: &Head)-> Vec<Base>{       
        let base = get_base(&head);
        return base.iter().map(|x| Base{
            name: x.to_string(),
            labels: get_base(&head)
        }).collect::<Vec<_>>();
    }

    fn get_base(head: &Head) -> Vec<String>{ 
        if head.mode == "feature" {
            return get_base_env(String::from("FEATURE_TARGETS"));
        } else if head.mode == "bugfix" {
            return get_base_env(String::from("BUGFIX"));
        }else if head.mode == "hotfix" {
            return get_base_env(String::from("HOTFIX_TARGETS"));
        } else {
            panic!("Head not in pattern");
        }
    }
    
    fn get_base_env(mode: String) -> Vec<String>{
       let response: Vec<String> = Vec::from_iter(env::var(mode).unwrap().split(",").map(String::from));
       return response;
    }
}

