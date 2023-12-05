use html_to_string_macro::html;
use std::fmt::{self, Display};

use crate::views::common::{Footer, Header};

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
                    <h1>"Error: "{self.status_code}</h1>
                    <p>{self.message.clone()}</p>
                </main>
            }
        )
    }
}

pub fn error_page(status_code: StatusCode, message: &str) -> String {
    let body = Body(vec![
        Box::new(Header),
        Box::new(ErrorPage {
            status_code,
            message: message.to_string(),
        }),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
