use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use crate::views::common::{Footer, Header};

pub struct Profile;

impl<'a> Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <h1>"Your Profile"</h1>
                </main>
            }
        )
    }
}

pub fn profile_page() -> String {
    let body = Body(vec![Box::new(Header), Box::new(Profile), Box::new(Footer)]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
