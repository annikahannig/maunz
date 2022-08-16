use std::collections::HashMap;
use std::fs;
use std::path;

use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, Utc};
use octocrab::models;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml;

#[derive(PartialEq, Debug)]
pub enum Status {
    Open,
    Closed,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    pub title: String,

    #[serde(deserialize_with = "parse_duration")]
    pub every: Duration,

    pub align: Option<bool>,
}

impl Meta {
    // parse issue meta
    fn parse(content: &str) -> Result<Meta> {
        let meta = serde_yaml::from_str(&content)?;
        Ok(meta)
    }

    // calculate next open
    fn next_open(&self, from: &DateTime<Utc>) -> DateTime<Utc> {
        let next = from.clone() + self.every;
        if self.align.unwrap_or(false) {
            match next.with_day(1) {
                Some(v) => v,
                None => next,
            }
        } else {
            next
        }
    }
}

fn parse_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let data = String::deserialize(deserializer)?;
    let len = data.len();
    let value: i64 = (&data[..len - 1]).parse().map_err(Error::custom)?;
    let suffix = &data[len - 1..];

    match suffix {
        "d" => Ok(Duration::days(value)),
        "w" => Ok(Duration::weeks(value)),
        "m" => Ok(Duration::days(value * 31)), // align month??
        _ => Err(Error::custom("invalid suffix, only d or w")),
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct State {
    pub github_id: Option<u64>,
    pub last_open: Option<DateTime<Utc>>,
    pub last_close: Option<DateTime<Utc>>,
}

impl State {
    pub fn status(&mut self) -> Status {
        if self.last_close < self.last_open {
            Status::Open
        } else {
            Status::Closed
        }
    }

    pub fn mark_open(&mut self) {
        if self.status() == Status::Closed {
            self.last_open = Some(Utc::now());
        }
    }

    pub fn mark_closed(&mut self) {
        if self.status() == Status::Open {
            self.last_close = Some(Utc::now());
        }
    }

    pub fn assign_github_issue(
        &mut self,
        issue: &models::issues::Issue,
    ) -> Result<()> {
        self.github_id = Some(u64::try_from(issue.number)?);
        Ok(())
    }
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
        None => "no_id".to_owned(),
    }
}

impl Issue {
    // parse an issue from file content
    fn parse(content: String) -> Result<Issue> {
        let parts: Vec<&str> = content.splitn(2, "---").collect();
        let meta = Meta::parse(parts[0])?;

        let issue = Issue {
            meta: meta,
            text: parts[1].to_owned(),
        };

        Ok(issue)
    }

    // from_file reads an issue file and parses it
    fn from_file(filename: &String) -> Result<Issue> {
        let content = fs::read_to_string(filename)?;
        Issue::parse(content)
    }

    // needs open checks if the last open was before
    // the duration window. When aligned we use the first of
    // the month as the last open.
    pub fn needs_open(&self, state: &State) -> bool {
        if state.last_close.is_none() {
            return false;
        }
        let next = self.meta.next_open(&state.last_close.unwrap());
        Utc::now() > next
    }
}

// A Repo contains a mapping of issue ids to
// parsed issues
pub type Repo = HashMap<String, Issue>;

// from_path loads all issues from a given path
// and returns an issue repo.
pub fn from_path(path: String) -> Result<Repo> {
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
    use crate::issue::{self, Issue, Meta, State, Status};
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    fn parse_meta() {
        let data = "
            title: yay!
            every: 10d
        ";
        let meta = Meta::parse(data).unwrap();
        assert_eq!(meta.title, "yay!");
        assert_eq!(meta.every, Duration::days(10));
    }

    #[test]
    fn next_open_unaligned() {
        let meta = Meta {
            title: "".to_string(),
            align: None,
            every: Duration::days(10),
        };
        let t0 = Utc.ymd(2000, 1, 12).and_hms(0, 1, 1);
        assert_eq!(meta.next_open(&t0), Utc.ymd(2000, 1, 22).and_hms(0, 1, 1));
    }

    #[test]
    fn next_open_aligned() {
        let meta = Meta {
            title: "".to_string(),
            align: Some(true),
            every: Duration::days(31),
        };
        let t0 = Utc.ymd(2000, 1, 1).and_hms(0, 1, 1);
        assert_eq!(meta.next_open(&t0), Utc.ymd(2000, 2, 1).and_hms(0, 1, 1));
    }

    #[test]
    fn parse_issue() {
        let data = String::from(
            "
            title: test title
            every: 10d
            ---
            - [ ] do things
            - [ ] do more
        ",
        );
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
    fn from_path() {
        let issues = issue::from_path(String::from("./example")).unwrap();
        for (id, issue) in issues {
            println!("ID: {}, Issue: {}", id, issue.meta.title);
        }
    }

    #[test]
    fn status() {
        let mut state = State {
            last_open: None,
            last_close: None,
            github_id: None,
        };
        assert_eq!(state.status(), Status::Closed);

        state.last_open = Some(Utc::now());
        assert_eq!(state.status(), Status::Open);

        state.last_close = Some(Utc::now() - Duration::days(5));
        assert_eq!(state.status(), Status::Open);

        state.last_open = Some(Utc::now() - Duration::days(10));
        assert_eq!(state.status(), Status::Closed);
    }
}
