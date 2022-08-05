
use octocrab::{
    Octocrab,
    params,
    models,
};

use crate::env;

pub async fn fetch_issues() -> Result<Vec<models::issues::Issue>, octocrab::Error> {
    let octocrab = Octocrab::builder()
        .personal_token(env::github_token())
        .build()?;

    let mut page = octocrab
        .issues("annikahannig", "life")
        .list()
        .state(params::State::All)
        .per_page(100)
        .send()
        .await?;

    let mut issues: Vec<models::issues::Issue> = Vec::new();

    // Get from all pages
    loop {
        for issue in &page {
            // Skip any issues that are PRs.
            if issue.pull_request.is_some() {
                continue;
            }
            issues.push(issue.clone());
        }
        page = match octocrab
            .get_page::<models::issues::Issue>(&page.next)
            .await?
        {
            Some(next_page) => next_page,
            None => break,
        }
    }

    Ok(issues)
}