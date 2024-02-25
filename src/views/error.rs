use html_to_string_macro::html;
use std::fmt::{self, Display};

use crate::{
    models::user::ExpandedUser,
    views::common::{Footer, Header},
};

use super::{Body, Document, Head};
use warp::hyper::StatusCode;

pub struct ErrorPage {
    status_code: StatusCode,
    message: String,
}

impl Display for ErrorPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <section id="error">
                        <h1>"Error: "{self.status_code}</h1>
                        <p>{self.message.clone()}</p>
                    </section>
                </main>
            }
        )
    }
}

pub fn error_page(
    status_code: StatusCode,
    message: &str,
    expanded_user: Option<ExpandedUser>,
) -> String {
    let body = Body(vec![
        Box::new(Header { expanded_user }),
        Box::new(ErrorPage {
            status_code,
            message: message.to_string(),
        }),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head {
            title: "Digitheque: Error".to_string(),
            description: "Digitheque: Online Publishing! Draft and publish your custom magazines, pamphlets, and notes.".to_string()
        },
        body: &body,
    };
    format!("{}", html)
}
