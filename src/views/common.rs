use html_to_string_macro::html;
use std::fmt::{self, Display};

use crate::models::user::ExpandedUser;

use super::{Body, Document, Head};

pub struct Header {
    pub expanded_user: Option<ExpandedUser>,
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <header id="banner">
                    <div id="banner-wrapper" class="body-spacing">
                        <div id="titles">
                            <h1 id="website-title">"Digitheque"</h1>
                            <h2 id="website-subtitle">"Markdown zines for everyone"</h2>
                        </div>
                        <nav>
                            {
                                match &self.expanded_user {
                                    None => html! {
                                        <a href="/user/login">"Login"</a>
                                    },
                                    Some(user) => html! {
                                        <a href="/user">{&user.user.username}</a>
                                        <a href="/user/logout">"Logout"</a>
                                    }
                                }
                            }
                            <a href="/about">"About"</a>
                        </nav>
                    </div>
                </header>
            }
        )
    }
}

pub struct Footer;

impl Display for Footer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <footer>
                    <p>"Copyright Digitheque Ltd. 2023"</p>
                </footer>
            }
        )
    }
}

pub struct Landing;

impl Display for Landing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <h1>"Welcome"</h1>
                </main>
            }
        )
    }
}

pub fn landing_page() -> String {
    let body = Body(vec![
        Box::new(Header {
            expanded_user: None,
        }),
        Box::new(Landing),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
