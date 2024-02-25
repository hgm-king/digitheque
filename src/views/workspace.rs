use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use crate::{
    models,
    views::common::{Footer, Header},
    GLOBAL_PRELUDE,
};

#[derive(Clone)]
pub struct WorkspacePage {
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
}

impl Display for WorkspacePage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main id="workspace-container">
                    <aside>
                        <h2>"Details"</h2>
                        {self.workspace.workspace.details()}
                        <h2>"Actions"</h2>
                        {self.workspace.workspace.actions(false)}
                        <div class="hide-on-mobile">
                            <h3>"Subworkspaces"</h3>
                            {self.workspace.subworkspaces()}
                        </div>
                        <details class="show-on-mobile">
                            <summary>"Subworkspaces"</summary>
                            {self.workspace.subworkspaces()}
                        </details>
                    </aside>
                    <section id="workspace">
                        {self.workspace.workspace.execute_content(format!("{}\n{}", GLOBAL_PRELUDE, self.expanded_user.user.prelude.clone().unwrap_or(String::from(""))))}
                    </section>
                </main>
            }
        )
    }
}

#[derive(Clone)]
pub struct WorkspaceEdit {
    _expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
}

impl Display for WorkspaceEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main id="workspace-container">
                    <aside>
                        <h2>"Details"</h2>
                        {self.workspace.workspace.details()}
                        <h2>"Actions"</h2>
                        {self.workspace.workspace.actions(true)}
                        <div class="hide-on-mobile">
                            <h3>"Subworkspaces"</h3>
                            {self.workspace.subworkspaces()}
                        </div>
                        <details class="show-on-mobile">
                            <summary>"Subworkspaces"</summary>
                            {self.workspace.subworkspaces()}
                        </details>
                    </aside>
                    <section id="edit-workspace">
                        <h2>"Edit Workspace"</h2>
                        {self.workspace.workspace.edit_self_form()}
                    </section>
                </main>
            }
        )
    }
}

pub fn workspace_page(
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> String {
    let header = Header {
        expanded_user: Some(expanded_user.clone()),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(WorkspacePage {
        expanded_user,
        workspace: workspace.clone(),
    }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head {
            title: workspace.workspace.name,
            description: workspace.workspace.description
        },
        body: &body,
    };
    format!("{}", html)
}

pub fn edit_workspace_page(
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> String {
    let header = Header {
        expanded_user: Some(expanded_user.clone()),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(WorkspaceEdit {
        _expanded_user: expanded_user,
        workspace: workspace.clone(),
    }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head {
            title: workspace.workspace.name,
            description: workspace.workspace.description
        },
        body: &body,
    };
    format!("{}", html)
}
