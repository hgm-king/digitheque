pub mod auth;
pub mod common;
pub mod error;
pub mod user;
pub mod workspace;

use std::fmt::{self, Display};

use html_to_string_macro::html;

pub struct Document<'a> {
    pub head: &'a Head,
    pub body: &'a Body,
}

impl<'a> Display for Document<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <!DOCTYPE html>
                <html lang="en">
                    {self.head}
                    {self.body}
                </html>
            }
        )
    }
}

pub struct Head;

impl Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <head>
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <meta charset="utf-8" />
                    <title>"Digitheque"</title>
                    <link rel="stylesheet" href="/styles/style.css" />
                </head>
            }
        )
    }
}

/*
<meta name="viewport" content="width=device-width, initial-scale=1" />
                    <meta charset="utf-8" />
                    <title>"Digitheque"</title>
                    <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
                    <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
                    <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
                    <link rel="manifest" href="/site.webmanifest" />
                    <link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5" />
                    <meta name="msapplication-TileColor" content="#da532c" />
                    <meta name="theme-color" content="#ffffff" />
                    <meta property="og:title" content="Digitheque" />
                    <meta property="og:site_name" content="Digitheque" />
                    <meta property="og:url" content="https://digitheque.io" />
                    <meta property="og:description" content="A social network allowing for people to explore the internet through a user-provided graph of websites." />
                    <meta property="og:type" content="website" />
                    <meta property="og:image" content="https://digitheque.io/seo/rainbow-logo1.png" />
                    <link rel="stylesheet" href="/styles/style.css" />
                    <link rel="stylesheet" href="/mobile.css" media="screen and (max-width: 600px)" />
                    <script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script>
                    <script src="https://cdn.jsdelivr.net/npm/d3@7"></script>
                    <script src="/sal.js"></script>
                    <link rel="stylesheet" href="/sal.css" />
                    <script src="/background.js"></script>
*/
pub struct Body(pub Vec<Box<dyn Display>>);

impl Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let html = self
            .0
            .iter()
            .map(|item| format!("{}", item))
            .collect::<String>();
        write!(
            f,
            "{}",
            html! {
                <body>
                    {html}
                </body>
            }
        )
    }
}
