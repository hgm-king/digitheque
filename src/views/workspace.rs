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
                <main class="split">
                    <section class="">
                        <h1 class="workspace-name">{self.workspace.workspace.name.clone()}</h1>
                        <h2 class="workspace-description">{self.workspace.workspace.description.clone()}</h2>
                        <a href={format!("/workspace/{}/edit", self.workspace.workspace.id)}>"Edit me!"</a>
                        <div class="markdown">
                            {handle_workspace_type(&self.workspace.workspace)}
                        </div>
                    </section>
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
                <article>
                    <h4>
                        <a href={format!("/workspace/{}", workspace.id)}>{workspace.name.clone()}</a>
                    </h4>
                </article>
            }
        }).collect::<String>();

        write!(
            f,
            "{}",
            html! {
                {childs}
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
                    <section>
                        <h1 class="workspace-name">{self.workspace.workspace.name.clone()}</h1>
                        <h2 class="workspace-description">{self.workspace.workspace.description.clone()}</h2>
                        <h3>"Edit"</h3>
                        {handle_workspace_edit(&self.workspace.workspace)}
                    </section>
                </main>
            }
        )
    }
}

fn handle_workspace_type(workspace: &models::workspace::Workspace) -> String {
    match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
        models::workspace::WorkspaceType::Markdown => {
            let input = workspace.content.clone().unwrap_or(String::from(""));
            println!("{:?}", input);
            let md = bebop_lang::markdown::parser::parse_markdown(&input).unwrap_or(("", vec![]));
            println!("{:?}", md);
            md.1
                .into_iter()
                .map(bebop_lang::markdown::html::markdown_to_html)
                .collect()
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
