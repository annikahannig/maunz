

mod env;
mod issue;
mod state;
mod github;



#[tokio::main]
async fn main() -> Result<(), octocrab::Error> {
    let mut state = state::from_file(env::state_file()).unwrap();
    let repo = issue::from_path(env::issues_path()).unwrap();

    let gh = github::Github::new(
        env::github_token(),
        String::from("annikahannig"),
        String::from("life"),
    );

    // Sync state with repo
    for (id, issue) in repo {
        let mut issue_state = state.track_issue(&id);
        match issue_state.github_id {
            None => {
                // issue_state.github_id = gh.create_issue(&issue).await?;
                let github_issue = gh.create_issue(&issue).await?;
                issue_state.mark_open(github_issue.id.into_inner());
            },
            Some(id) => {
                let github_issue = gh.fetch_issue(id).await?;
                if github_issue.state == String::from("open") {
                    issue_state.is_open = true
                } else {
                    issue_state.is_open = false
                }

                println!("check state and check if needs reopen{}", id)
            },
        }
    }

    // Write state
    state.save(env::state_file()).unwrap();

    Ok(())
}
