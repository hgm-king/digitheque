use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use crate::{
    models::user::ExpandedUser,
    views::common::{Footer, Header},
};

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

pub fn profile_page(expanded_user: ExpandedUser) -> String {
    let header = Header {
        expanded_user: Some(expanded_user),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(Profile));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
