
use std::fs;
use std::io;
use std::path;
use std::error::Error;

use serde::{Serialize, Deserialize};
use serde_yaml;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub title: String,
    pub every: String,
}

impl Meta {
    // parse issue meta
    fn parse(content: &str) -> Result<Meta, serde_yaml::Error> {
        let meta = serde_yaml::from_str(&content)?;
        Ok(meta)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub is_open: bool,
    pub last_open: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Issue {
    pub id: String,
    pub meta: Meta,
    pub text: String,
}

fn id_from_filename(filename: &String) -> String {
    let path = path::Path::new(&filename);
    match path.file_name() {
        Some(name) => name.to_str().unwrap().to_owned(),
        None => "no_id".to_owned()
    }
}

impl Issue {

    fn parse(id: String, content: String) -> Result<Issue, Box<dyn Error>> { 
        let parts: Vec<&str> = content.splitn(2, "---").collect();
        let meta = Meta::parse(parts[0])?;

        let issue = Issue{
            id: id,
            meta: meta,
            text: parts[1].to_owned(),
        };

        Ok(issue)
    }
        
    fn from_file(filename: String) -> Result<Issue, Box<dyn Error>> {
        let id = id_from_filename(&filename);
        let content = fs::read_to_string(filename)?;
        Issue::parse(id, content)
    }
}


#[cfg(test)]
mod tests {
    use crate::issue::{Issue, Meta};
    
    #[test]
    fn parse_meta() {
        let data = "
            title: yay!
            every: 10d
        ";
        let meta = Meta::parse(data).unwrap(); 
        assert_eq!(meta.title, "yay!");
        assert_eq!(meta.every, "10d");
    }

    #[test]
    fn parse_issue() {
        let id = String::from("my_id.md");
        let data = String::from("
            title: test title
            every: 10d
            ---
            - [ ] do things
            - [ ] do more
        ");
        let issue = Issue::parse(id, data).unwrap();
        assert_eq!(issue.meta.title, "test title");
        println!("{}", issue.text);
    }

    #[test]
    fn from_file() {
        let filename = String::from("./example/clean_table.md");
        let issue = Issue::from_file(filename).unwrap();
        assert_eq!(issue.id, "clean_table.md");
        assert_eq!(issue.meta.title, "Maintain Work Environment");
        println!("{}", issue.text);
    }
}
