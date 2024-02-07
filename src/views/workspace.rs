use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use crate::{
    models,
    views::common::{Footer, Header},
    GLOBAL_PRELUDE,
};

pub struct Workspace {
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
}

impl Display for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <aside id="sub-workspaces">
                        {if self.workspace.workspace.parent_id != -1 {
                            html! {<h4><a href={format!("/workspace/{}", self.workspace.workspace.parent_id)}>"← Back to parent"</a></h4>}
                        } else {
                            String::from("")
                        }}
                        <h3>"Subworkspaces"</h3>
                        {WorkspaceChildren(self.workspace.children.clone())}
                        <h4>"Add new"</h4>
                        <form action={format!("/workspace/{}", self.workspace.workspace.id)} method="POST">
                            <fieldset>
                                <legend>"Workspace details"</legend>
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
                            </fieldset>
                        </form>
                    </aside>
                    <section id="workspace">
                        <a href={format!("/workspace/{}/edit", self.workspace.workspace.id)}>"Edit me!"</a>
                        <div class="markdown">
                            {handle_workspace_type(&self.expanded_user, &self.workspace.workspace)}
                        </div>
                    </section>
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
                <li>
                    <article>
                        <h5>
                            <a href={format!("/workspace/{}", workspace.id)}>{workspace.name.clone()}</a>
                        </h5>
                    </article>
                </li>
            }
        }).collect::<String>();

        write!(
            f,
            "{}",
            html! {
                <ul>
                    {childs}
                </ul>
            }
        )
    }
}

pub struct WorkspaceEdit {
    workspace: models::workspace::WorkspaceWithChildren,
}

impl Display for WorkspaceEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <section>
                        <h3>"Edit"</h3>
                        {handle_workspace_edit(&self.workspace.workspace)}
                    </section>
                </main>
            }
        )
    }
}

fn handle_workspace_type(
    expanded_user: &models::user::ExpandedUser,
    workspace: &models::workspace::Workspace,
) -> String {
    match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
        _ => {
            let input = workspace.content.clone().unwrap_or(String::from(""));
            println!("{:?}", input);
            let md = bebop_lang::markdown::parser::parse_markdown(&input).unwrap_or(("", vec![]));
            let mut env = bebop_lang::lisp::env::init_env();
            println!("{:?}", md);
            let lisp: String =
                md.1.into_iter()
                    .map(bebop_lang::markdown::lisp::markdown_to_lisp)
                    .collect();

            let input = &format!(
                r#"
            {}
            {}
            (def [title] "{}")
            (def [description] "{}")
            (def [updated-at] "{}")
            {}
            "#,
                GLOBAL_PRELUDE,
                expanded_user.user.prelude.clone().unwrap_or_default(),
                workspace.name,
                workspace.description,
                workspace.updated_at.unwrap_or_default(),
                lisp
            );

            println!("{}", input);

            let v = bebop_lang::lisp::lisp(&mut env, input);
            v
        }
    }
}

fn handle_workspace_edit(workspace: &models::workspace::Workspace) -> String {
    html! {
        <form action={format!("/workspace/{}/edit", workspace.id)} method="POST">
            <input type="text" name="name" value={workspace.name.clone()} />
            <input type="text" name="description" value={workspace.description.clone()} />
            {match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
                _ => html! {
                    <textarea name="content">{workspace.content.clone().unwrap_or(String::from("# Edit me to get started!\nMake sure to save using the button at the bottom.\n"))}</textarea>
                },
            }}
            <button type="submit">"Submit"</button>
        </form>
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
    body.0.push(Box::new(Workspace {
        expanded_user,
        workspace,
    }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}

pub fn edit_workspace_page(
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> String {
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
