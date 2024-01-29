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
                    <span id="logo"><a href="/" title="Digitheque.io">"Digitheque"</a></span>
                    <nav>
                        <ul>
                            {
                                match &self.expanded_user {
                                    None => html! {
                                        <li><a href="/user/login">"Login"</a></li>
                                    },
                                    Some(user) => html! {
                                        <li><a href="/user">{&user.user.username}</a></li>
                                        <li><a href="/user/logout">"Logout"</a></li>
                                    }
                                }
                            }
                            <li><a href="/">"About"</a></li>
                        </ul>
                    </nav>
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
                    <p><a href="https://github.com/hgm-king/digitheque">"Code"</a></p>
                    <p>"Built by "<a href="mailto:hgmaxwellking@gmail.com">"HG King"</a></p>
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
                    <h1>"Welcome to the Digitheque"</h1>
                    <p>"A decentralized magazine platform for everyone. Aiming to make the internet more pleasant and accessible."</p>
                    <p>"This platform allows you to write to your heart's desire. We offer a decent Markdown editor for you to do this. Your writing
                    happens in workspaces which contain both your writing and sub-workspaces. Each user is given a root workspace."</p>
                    <p>"We intend to allow users to post their writing through RSS or another sydicated protocol to help get their work out there."</p>
                    <a href="/user/signup">"Get started by signing up and exploring!"</a>
                    <p>"We are in a very early stage of development, please contact me through my email found in the footer."</p>
                </main>
            }
        )
    }
}

pub fn landing_page(expanded_user: Option<ExpandedUser>) -> String {
    let body = Body(vec![
        Box::new(Header { expanded_user }),
        Box::new(Landing),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
