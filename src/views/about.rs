use html_to_string_macro::html;
use std::fmt::{self, Display};

use crate::views::common::{Footer, Header};

use super::{Body, Document, Head};
use warp::hyper::StatusCode;

pub struct About {
    status_code: StatusCode,
    message: String,
}

impl Display for About {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <h1>"About the digitheque"</h1>
                    <p>"A digitheque is an interactive notebook that has a 
                    focus on markdown and html."</p>
                </main>
            }
        )
    }
}

pub fn about_page() -> String {
    let body = Body(vec![
        Box::new(Header {
            expanded_user: None
        }),
        Box::new(About {
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
