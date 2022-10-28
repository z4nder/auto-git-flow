pub mod head {
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Head {
        pub name: String,
        pub mode: String,
        pub project: String,
        pub id: String,
        pub description: String
    }

    pub fn factory(head: String) -> Head{
        let mode_name: Vec<String> = head.split("/").map(|s| s.to_string()).collect();
        let mode = &mode_name[0];
        let name = &mode_name[1];

        let project_id_description: Vec<String> = name.split("-").map(|s| s.to_string()).collect();
        let project = &project_id_description[0];
        let id = &project_id_description[1];

        let description = &project_id_description[2..project_id_description.len()];

        return Head {
            name: head,
            mode: mode.to_string(),
            project: project.to_string(),
            id: id.to_string(),
            description: description.join(" "),
        };
    }
}

