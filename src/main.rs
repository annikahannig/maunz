use anyhow::Result;

mod env;
mod github;
mod issue;
mod state;


#[tokio::main]
async fn main() -> Result<()> {
    let gh = github::Github::new(
        env::github_token()?,
        env::github_repo()?,
    );
    Ok(())
}


/*
#[tokio::main]
async fn main() -> Result<(), octocrab::Error> {
    let mut state = state::from_file(env::state_file()).unwrap();
    let repo = issue::from_path(env::issues_path()).unwrap();


    // Sync state with repo
    for (id, issue) in repo {
        let issue_state = state.track_issue(&id);
        match issue_state.github_id {
            None => {
                let github_issue = gh.create_issue(&issue).await?;
                issue_state.assign_github_issue(&github_issue);
                issue_state.mark_open();
            }
            Some(id) => {
                let github_issue = gh.fetch_issue(id).await?;
                if github_issue.state == String::from("open") {
                    issue_state.mark_open()
                } else {
                    issue_state.mark_closed()
                }

                if issue.needs_open(issue_state) {
                    gh.reopen_issue(id).await?;
                    issue_state.mark_open();
                }
            }
        }
    }

    // Write state
    state.save(env::state_file()).unwrap();

    Ok(())
}
*/
