use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use crate::{
    models,
    views::common::{Footer, Header},
    views::workspace,
};

pub fn profile_page(
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> String {
    workspace::workspace_page(expanded_user, workspace)
}

pub fn style_edit_page(
    expanded_user: models::user::ExpandedUser,
    message: Option<String>,
) -> String {
    let header = Header {
        expanded_user: Some(expanded_user.clone()),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(StyleEdit {
        expanded_user,
        message,
    }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head {
            title: "Digitheque".to_string(),
            description: "Digitheque: Online Publishing! Draft and publish your custom magazines, pamphlets, and notes.".to_string()
        },
        body: &body,
    };
    format!("{}", html)
}

pub fn prelude_edit_page(
    expanded_user: models::user::ExpandedUser,
    message: Option<String>,
) -> String {
    let header = Header {
        expanded_user: Some(expanded_user.clone()),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(PreludeEdit {
        expanded_user,
        message,
    }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head {
            title: "Digitheque".to_string(),
            description: "Digitheque: Online Publishing! Draft and publish your custom magazines, pamphlets, and notes.".to_string()
        },
        body: &body,
    };
    format!("{}", html)
}

pub struct StyleEdit {
    expanded_user: models::user::ExpandedUser,
    message: Option<String>,
}

impl Display for StyleEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <section id="edit-style">
                        <h3>"Edit Style"</h3>
                        <p>{self.message.clone().unwrap_or_default()}</p>
                            <form action="/style" method="POST">
                                <textarea name="style">{self.expanded_user.user.style.clone().unwrap_or(String::from("# Edit me to get started!\nMake sure to save using the button at the bottom.\n"))}</textarea>
                                <button type="submit">"Submit"</button>
                            </form>
                    </section>
                </main>
            }
        )
    }
}

pub struct PreludeEdit {
    expanded_user: models::user::ExpandedUser,
    message: Option<String>,
}

impl Display for PreludeEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <section id="edit-prelude">
                        <h3>"Edit Prelude"</h3>
                        <p>"Here is where you can specify global functions and values for your Bebop code. Learn more about Bebop by reading the "<a href="/bebop">"spec"</a>". "</p>
                        <p>"This code will run each time your workspaces get rendered."</p>
                        <p>{self.message.clone().unwrap_or_default()}</p>
                        <form action="/prelude" method="POST">
                            <textarea name="prelude">{self.expanded_user.user.prelude.clone().unwrap_or(String::from("# Edit me to get started!\nMake sure to save using the button at the bottom.\n"))}</textarea>
                            <button type="submit">"Submit"</button>
                        </form>
                    </section>
                </main>
            }
        )
    }
}
