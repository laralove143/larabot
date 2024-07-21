use anyhow::{bail, Result};
use sentry::{capture_message, Hub};

use crate::option_ext::OptionExt;

#[derive(Debug)]
pub struct FeedbackClient {
    client: reqwest::Client,
    token: String,
}

impl FeedbackClient {
    #[must_use]
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            token,
        }
    }

    pub async fn create_user_feedback(&self, username: String, content: String) -> Result<()> {
        let dsn = Hub::with(|hub| hub.client().and_then(|c| c.dsn().cloned())).ok()?;
        let organization_id = dsn.host().trim_start_matches('o').split_once('.').ok()?.0;
        let project_id = dsn.project_id();

        let event_id = capture_message("User Feedback", sentry::Level::Info);

        let body = Body {
            event_id: event_id.simple().to_string(),
            name: username,
            email: "not_available@example.com".to_owned(),
            comments: content,
        };

        let response = self
            .client
            .post(format!(
                "https://sentry.io/api/0/projects/{organization_id}/{project_id}/user-feedback/",
            ))
            .json(&body)
            .bearer_auth(&self.token)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(
                "failed to create user feedback: {body}",
                body = response.text().await?
            );
        }

        Ok(())
    }
}

#[derive(serde::Serialize)]
struct Body {
    event_id: String,
    name: String,
    email: String,
    comments: String,
}

#[cfg(test)]
mod tests {
    use std::env;

    use anyhow::Result;

    use crate::sentry_user_feedback::FeedbackClient;

    #[tokio::test]
    async fn test_create_user_feedback() -> Result<()> {
        let _sentry_guard = crate::tracing::tests::init()?;

        let client = FeedbackClient::new(env::var("SENTRY_TOKEN")?);

        client
            .create_user_feedback("username".to_owned(), "content".to_owned())
            .await?;

        Ok(())
    }
}
