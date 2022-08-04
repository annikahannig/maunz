

pub fn github_token() -> String {
    std::env::var("NOLF_GITHUB_TOKEN")
        .expect("GITHUB_TOKEN env variable is required")
}

pub fn issues() -> String {
    std::env::var("NOLF_ISSUES")
        .expect("NOLF_ISSUES should contain the path to the issues")
}

