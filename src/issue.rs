
use std::fs;
use std::path;
use std::error::Error;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yaml;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Clone, Debug)]
pub struct Issue {
    pub meta: Meta,
    pub text: String,
}

// id_from_filename derives the issue ID from a filename
fn id_from_filename(filename: &String) -> String {
    let path = path::Path::new(&filename);
    match path.file_name() {
        Some(name) => name.to_str().unwrap().to_owned(),
        None => "no_id".to_owned()
    }
}

impl Issue {

    // parse an issue from file content
    fn parse(content: String) -> Result<Issue, Box<dyn Error>> { 
        let parts: Vec<&str> = content.splitn(2, "---").collect();
        let meta = Meta::parse(parts[0])?;

        let issue = Issue{
            meta: meta,
            text: parts[1].to_owned(),
        };

        Ok(issue)
    }
        
    // from_file reads an issue file and parses it
    fn from_file(filename: &String) -> Result<Issue, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Issue::parse(content)
    }
}

// A Repo contains a mapping of issue ids to
// parsed issues
type Repo = HashMap<String, Issue>;

// load_path loads all issues from a given path
// and returns an issue repo.
pub fn load_path(path: String) -> Result<Repo, Box<dyn Error>> {
    let mut issues: Repo = HashMap::new();
    let paths = fs::read_dir(&path::Path::new(&path))?;
    
    for entry in paths {
        let path = entry.unwrap().path().to_str().unwrap().to_owned();
        let issue = Issue::from_file(&path)?;
        let id = id_from_filename(&path);
        issues.insert(id, issue.clone());
    }

    Ok(issues)
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
        let data = String::from("
            title: test title
            every: 10d
            ---
            - [ ] do things
            - [ ] do more
        ");
        let issue = Issue::parse(data).unwrap();
        assert_eq!(issue.meta.title, "test title");
        println!("{}", issue.text);
    }

    #[test]
    fn from_file() {
        let filename = String::from("./example/clean_table.md");
        let issue = Issue::from_file(&filename).unwrap();
        assert_eq!(issue.meta.title, "Maintain Work Environment");
        println!("{}", issue.text);
    }

    #[test]
    fn load_path() {
        let issues = crate::issue::load_path(String::from("./example")).unwrap();
        for (id, issue) in issues {
            println!("ID: {}, Issue: {}", id, issue.meta.title); 
        }
    }
}
