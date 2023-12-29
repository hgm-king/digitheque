use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use crate::{
    models,
    views::common::{Footer, Header},
};

pub struct Workspace {
    workspace: models::workspace::WorkspaceWithChildren
}

impl Display for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <h1>{self.workspace.workspace.name.clone()}</h1>
                    <p>{self.workspace.workspace.description.clone()}</p>
                    <a href={format!("/workspace/{}/edit", self.workspace.workspace.id)}>"Edit me!"</a>
                    {handle_workspace_type(&self.workspace.workspace)}
                    <p>"We have "{self.workspace.children.len()}" children"</p>
                    {WorkspaceChildren(self.workspace.children.clone())}
                    <h3>"Add new"</h3>
                    <form action={format!("/workspace/{}", self.workspace.workspace.id)} method="POST">
                        <label>
                            <span>"Name"</span>
                            <input type="text" name="name" required max=64 />
                        </label>
                        <label>
                            <span>"Description"</span>
                            <input type="text" name="description" required max=248 />
                        </label>
                        <input type="hidden" name="type_id" value=2 />
                        
                        <button type="submit">"Add new"</button>
                    </form>
                </main>
            }
        )
    }
}

pub struct WorkspaceChildren(Vec<models::workspace::Workspace>);

impl Display for WorkspaceChildren {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let childs = self.0.iter().map(|workspace| {
            html! {
                <h2><a href={format!("/workspace/{}", workspace.id)}>{workspace.name.clone()}</a></h2>
                {handle_workspace_type(workspace)}
            }
        }).collect::<String>();

        write!(
            f,
            "{}",
            html! {
                <section>
                    {childs}
                </section>
            }
        )
    }
}

pub struct WorkspaceEdit {
    workspace: models::workspace::WorkspaceWithChildren
}

impl Display for WorkspaceEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <h1>{self.workspace.workspace.name.clone()}</h1>
                    <p>{self.workspace.workspace.description.clone()}</p>
                    <h3>"Edit"</h3>
                    {handle_workspace_edit(&self.workspace.workspace)}
                </main>
            }
        )
    }
}

fn handle_workspace_type(workspace: &models::workspace::Workspace) -> String {
    match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
        models::workspace::WorkspaceType::Markdown => match bebop_lang::markdown::markdown_to_html(&workspace.content.clone().unwrap_or(String::from(""))) {
            Ok(s) => s,
            Err(s) => s
        },
        models::workspace::WorkspaceType::Root => html! {
            <span>"How did this get here??"</span>
        },
    }
}

fn handle_workspace_edit(workspace: &models::workspace::Workspace) -> String {
    html! {
        <form action={format!("/workspace/{}/edit", workspace.id)} method="POST">
            <input type="hidden" name="name" value={workspace.name.clone()} />
            <input type="hidden" name="description" value={workspace.description.clone()} />
            {match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
                models::workspace::WorkspaceType::Markdown => html! {
                    <textarea name="content">{workspace.content.clone().unwrap_or(String::from("Missing data!"))}</textarea>
                },
                models::workspace::WorkspaceType::Root => html! {
                    <span>"How did this get here??"</span>
                },
            }}
            <button type="submit">"Submit"</button>
        </form>
    }
}

pub fn workspace_page(expanded_user: models::user::ExpandedUser, workspace: models::workspace::WorkspaceWithChildren,) -> String {
    let header = Header {
        expanded_user: Some(expanded_user),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(Workspace { workspace }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}

pub fn edit_workspace_page(expanded_user: models::user::ExpandedUser, workspace: models::workspace::WorkspaceWithChildren,) -> String {
    let header = Header {
        expanded_user: Some(expanded_user),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(WorkspaceEdit { workspace }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
