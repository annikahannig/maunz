use octocrab::{models, Octocrab};

use crate::issue;

pub struct Github {
    token: String,
    org: String,
    project: String,
}

impl Github {
    pub fn new(token: String, org: String, project: String) -> Github {
        Github {
            token: token,
            org: org,
            project: project,
        }
    }

    // create issue will create a new github issue
    pub async fn create_issue(
        &self,
        issue: &issue::Issue,
    ) -> Result<models::issues::Issue, octocrab::Error> {
        let octocrab = Octocrab::builder()
            .personal_token(self.token.clone())
            .build()?;
        octocrab
            .issues(self.org.clone(), self.project.clone())
            .create(&issue.meta.title)
            .body(&issue.text)
            .send()
            .await
    }

    // reopen_issue will update the state of an issue and
    // set it to open.
    pub async fn reopen_issue(
        &self,
        github_id: u64,
    ) -> Result<models::issues::Issue, octocrab::Error> {
        let octocrab = Octocrab::builder()
            .personal_token(self.token.clone())
            .build()?;
        
        octocrab
            .issues(self.org.clone(), self.project.clone())
            .update(github_id)
            .state(models::IssueState::Open)
            .send()
            .await
    }

    // fetch a single issue by ID
    pub async fn fetch_issue(
        &self,
        id: u64,
    ) -> Result<models::issues::Issue, octocrab::Error> {
        let octocrab = Octocrab::builder()
            .personal_token(self.token.clone())
            .build()?;
        octocrab
            .issues(self.org.clone(), self.project.clone())
            .get(id)
            .await
    }
}
