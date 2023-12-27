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
                    <p>"We have "{self.workspace.children.len()}" children"</p>
                    {handle_workspace_type(&self.workspace.workspace)}
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
                        <label>
                            <span>"Type"</span>
                            <select name="type_id">
                                <option value="2">"Notepad"</option>
                                <option value="3">"Todo"</option>
                                <option value="4">"Link"</option>
                                <option value="5">"Image"</option>
                            </select>
                        </label>
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

fn handle_workspace_type(workspace: &models::workspace::Workspace) -> String {
    match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
        models::workspace::WorkspaceType::Image => html! {
            <image src={workspace.img_url.clone().unwrap_or(String::from("/img/missing.png"))} />
        },
        models::workspace::WorkspaceType::Notepad => html! {
            <textarea>{workspace.content.clone().unwrap_or(String::from("Missing data!"))}</textarea>
        },
        models::workspace::WorkspaceType::Link => html! {
            <a href={workspace.img_url.clone().unwrap_or(String::from("/img/missing.png"))}>{workspace.name.clone()}</a>
        },
        models::workspace::WorkspaceType::Todo => html! {
            <label>
            <span>{workspace.name.clone()}</span>
            <input type="checkbox" checked={workspace.todo_state.unwrap_or(false)} />
            </label>
        },
        models::workspace::WorkspaceType::Root => html! {
            <span>"How did this get here??"</span>
        },
    }
}

fn handle_workspace_edit(workspace: &models::workspace::Workspace) -> String {
    html! {
        <form action={format!("/workspace/{}/edit", workspace.id)} method="POST">
            {match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
                models::workspace::WorkspaceType::Image => html! {
                    <image src={workspace.img_url.clone().unwrap_or(String::from("/img/missing.png"))} />
                },
                models::workspace::WorkspaceType::Notepad => html! {
                    <textarea>{workspace.content.clone().unwrap_or(String::from("Missing data!"))}</textarea>
                },
                models::workspace::WorkspaceType::Link => html! {
                    <a href={workspace.img_url.clone().unwrap_or(String::from("/img/missing.png"))}>{workspace.name.clone()}</a>
                },
                models::workspace::WorkspaceType::Todo => html! {
                    <label>
                    <span>{workspace.name.clone()}</span>
                    <input type="checkbox" checked={workspace.todo_state.unwrap_or(false)} />
                    </label>
                },
                models::workspace::WorkspaceType::Root => html! {
                    <span>"How did this get here??"</span>
                },
            }}
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
