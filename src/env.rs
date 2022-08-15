// github_token returns the github access token
// from the environment.
pub fn github_token() -> String {
    std::env::var("MAUNZ_GITHUB_TOKEN")
        .expect("GITHUB_TOKEN env variable is required")
}

// state_file returns the path to the state file
// from the environment.
pub fn state_file() -> String {
    std::env::var("MAUNZ_STATE")
        .expect("MAUNZ_STATE should contain a path to the state file")
}

// issues_path returns the path to all issues
pub fn issues_path() -> String {
    std::env::var("MAUNZ_ISSUES")
        .expect("MAUNZ_ISSUES should contain the path to the issues")
}
