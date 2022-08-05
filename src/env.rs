

// github_token returns the github access token
// from the environment.
pub fn github_token() -> String {
    std::env::var("NOLF_GITHUB_TOKEN")
        .expect("GITHUB_TOKEN env variable is required")
}

// state_file returns the path to the state file
// from the environment.
pub fn state_file() -> String {
   std::env::var("NOLF_STATE")
       .expect("NOLF_STATE should contain a path to the state file")
}

// issues_path returns the path to all issues
pub fn issues_path() -> String {
    std::env::var("NOLF_ISSUES")
        .expect("NOLF_ISSUES should contain the path to the issues")
}

