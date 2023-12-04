use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use warp::hyper::StatusCode;

pub struct ErrorPage<'a> {
    status_code: StatusCode,
    message: &'a str,
}

impl<'a> Display for ErrorPage<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <section class="error">
                    <h2>"Error: "{self.status_code}</h2>
                    <h4>{self.message}</h4>
                </section>
            }
        )
    }
}

pub fn error_page(status_code: StatusCode, message: &str) -> String {
    let body = Body(vec![ErrorPage {
        status_code,
        message,
    }]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
