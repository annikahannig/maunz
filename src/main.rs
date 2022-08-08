

mod env;
mod issue;
mod state;
mod github;

#[tokio::main]
async fn main() -> Result<(), octocrab::Error> {
    let state = state::from_file(env::state_file()).unwrap();
    let repo = issue::from_path(env::issues_path()).unwrap();

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
