

mod env;
mod issue;
mod state;
mod github;

#[tokio::main]
async fn main() -> Result<(), octocrab::Error> {
    let mut state = state::from_file(env::state_file()).unwrap();
    let repo = issue::from_path(env::issues_path()).unwrap();

    // Fetch all issues from github
    // let github_issues = github::fetch_issues().await?;

    // Sync state with repo
    for (id, issue) in repo {
        let mut issue_state = state.track_issue(&id);
        // issue_state.mark_open(93109)
    }

    /*
    let issues = github::fetch_issues().await?;

    for issue in issues {
        println!("{}: {}, ({})", issue.number, issue.title, issue.state);
        match issue.body {
            Some(body) =>
                println!("{}", body),
            _ => ()
        }
    }
    */

    // Write state
    state.save(env::state_file()).unwrap();

    Ok(())
}
