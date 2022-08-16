use anyhow::{Error, Result};
use thiserror::Error;

use crate::github::Repo;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("missing environment variable {name:?}: {msg}")]
    Missing { name: String, msg: String },

    #[error("github repo format invalid, must be <org>:<repo>")]
    GithubRepoInvalid,
}

fn getenv(name: &str, msg: &str) -> Result<String> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(std::env::VarError::NotPresent) => Err(ConfigError::Missing {
            name: name.to_string(),
            msg: msg.to_string(),
        }
        .into()),
        Err(e) => Err(Error::from(e)),
    }
}

// github_token returns the github access token
// from the environment.
pub fn github_token() -> Result<String> {
    getenv("MAUNZ_GITHUB_TOKEN", "a github access token")
}

// github_repo retrieves the github repo as a tuple
pub fn github_repo() -> Result<Repo> {
    let var = getenv(
        "MAUNZ_GITHUB_REPO",
        "the github org and repo in format <org>:<repo>",
    )?;
    match var.split_once(":") {
        Some((org, repo)) => Ok(Repo(org.to_string(), repo.to_string())),
        None => Err(ConfigError::GithubRepoInvalid.into()),
    }
}

// state_file returns the path to the state file
// from the environment.
pub fn state_file() -> Result<String> {
    getenv("MAUNZ_STATE", "a path to the state file")
}

// issues_path returns the path to all issues
pub fn issues_path() -> Result<String> {
    getenv("MAUNZ_ISSUES", "a path to the issue files")
}
