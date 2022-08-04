

use octocrab::{
    Octocrab,
    params,
    models::issues::Issue,
};

mod env;


async fn fetch_issues() -> Result<(), octocrab::Error> {
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


    loop {
        for issue in &page {
            println!("{}", issue.title);
        }
        page = match octocrab
            .get_page::<Issue>(&page.next)
            .await?
        {
            Some(next_page) => next_page,
            None => break,
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() -> octocrab::Result<()> {
    fetch_issues().await?;

    Ok(())
}
