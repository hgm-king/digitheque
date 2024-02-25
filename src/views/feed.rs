use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use crate::{
    models,
    views::common::{Footer, Header},
    GLOBAL_PRELUDE,
};

pub struct FeedWorkspacePage {
    pub workspace: models::feed::FeedWorkspace,
}

impl Display for FeedWorkspacePage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main id="workspace-container">
                    <section id="workspace-feed">
                        {self.workspace.workspace.execute_content(format!("{}\n{}", GLOBAL_PRELUDE, self.workspace.user.prelude.clone().unwrap_or(String::from(""))))}
                    </section>
                </main>
            }
        )
    }
}

pub fn workspace_page(
    expanded_user: Option<models::user::ExpandedUser>,
    workspace: models::feed::FeedWorkspace
) -> String {
    let header = Header {
        expanded_user: expanded_user.clone(),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(FeedWorkspacePage {
        workspace: workspace.clone(),
    }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head {
            title: workspace.workspace.name,
            description: workspace.workspace.description.clone()
        },
        body: &body,
    };
    format!("{}", html)
}