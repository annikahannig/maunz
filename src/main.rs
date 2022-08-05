

mod env;
mod state;
mod github;

#[tokio::main]
async fn main() -> Result<(), octocrab::Error> {
    let issues = github::fetch_issues().await?;

    for issue in issues {
        println!("{}: {}, ({})", issue.number, issue.title, issue.state);
        match issue.body_text {
            Some(body) =>
                println!("{}", body),
            _ => ()
        }
    }

    Ok(())
}
