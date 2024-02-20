pub mod api;
pub mod config;
pub mod db_conn;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;
pub mod views;

#[macro_use]
extern crate diesel;

use models::user::ExpandedUser;
use std::sync::Arc;
use warp::{hyper::StatusCode, reject, Rejection, Reply};

#[derive(Debug)]
struct NotFound;
impl reject::Reject for NotFound {}

#[derive(Debug)]
struct NotAuthorized;
impl reject::Reject for NotAuthorized {}

#[derive(Debug)]
struct OldCookie;
impl reject::Reject for OldCookie {}

#[derive(Debug)]
pub struct ResourceError {
    message: String,
}
impl reject::Reject for ResourceError {}

#[derive(Debug)]
pub struct ServerError {
    message: String,
}
impl reject::Reject for ServerError {}

#[derive(Clone, Debug)]
pub struct ExpandedUserRejection {
    expanded_user: Option<ExpandedUser>,
}
impl reject::Reject for ExpandedUserRejection {}

#[derive(Clone, Debug)]
pub struct Context {
    pub config: Arc<config::Config>,
    pub db_conn: Arc<db_conn::DbConn>,
}

impl Context {
    pub fn new(config: Arc<config::Config>, db_conn: Arc<db_conn::DbConn>) -> Self {
        Context {
            config: config,
            db_conn,
        }
    }
}

pub async fn handle_rejections(err: Rejection) -> Result<impl Reply, Rejection> {
    let expanded_user = err
        .find::<ExpandedUserRejection>()
        .map(|eu| eu.clone().expanded_user)
        .unwrap_or(None);

    tracing::error!("{:?}", err);
    tracing::error!("Handling rejection for user {:?}", expanded_user);

    if expanded_user.is_some() {
        if let Some(e) = err.find::<ResourceError>() {
            let code = StatusCode::BAD_REQUEST;
            let html = views::error::error_page(code, &e.message, expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<NotAuthorized>() {
            let code = StatusCode::FORBIDDEN;
            let html =
                views::error::error_page(code, "You are not authorized to do this", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        }
        // else if let Some(_) = err.find::<OldCookie>() {
        //     Ok(Box::new(warp::reply::with_header(
        //         warp::reply::html(views::body::index("Your session has expired")),
        //         "Set-Cookie",
        //         format!("session=; Path=/"),
        //     )))
        // }
        else if let Some(_) = err.find::<reject::MissingCookie>() {
            let code = StatusCode::FORBIDDEN;
            let html = views::error::error_page(code, "You are not logged in", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(NotFound) = err.find::<NotFound>() {
            let code = StatusCode::NOT_FOUND;
            let html =
                views::error::error_page(code, "We could not locate this resource", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
            let message = e.to_string();
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, &message, expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
            let code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
            let html = views::error::error_page(code, "UNSUPPORTED MEDIA TYPE", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
            let code = StatusCode::METHOD_NOT_ALLOWED;
            let html = views::error::error_page(code, "METHOD NOT ALLOWED", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else {
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, "RESOURCE NOT FOUND", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        }
    } else {
        if let Some(e) = err.find::<ResourceError>() {
            let code = StatusCode::BAD_REQUEST;
            let html = views::error::error_page(code, &e.message, expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<NotAuthorized>() {
            let code = StatusCode::FORBIDDEN;
            let html =
                views::error::error_page(code, "You are not authorized to do this", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        }
        // else if let Some(_) = err.find::<OldCookie>() {
        //     Ok(Box::new(warp::reply::with_header(
        //         warp::reply::html(views::body::index("Your session has expired")),
        //         "Set-Cookie",
        //         format!("session=; Path=/"),
        //     )))
        // }
        // else if let Some(_) = err.find::<reject::MissingCookie>() {
        //     let code = StatusCode::FORBIDDEN;
        //     let html = views::error::error_page(code, "You are not logged in", expanded_user);
        //     Ok(warp::reply::with_status(warp::reply::html(html), code))
        // }
        else if let Some(NotFound) = err.find::<NotFound>() {
            let code = StatusCode::NOT_FOUND;
            let html =
                views::error::error_page(code, "We could not locate this resource", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
            let message = e.to_string();
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, &message, expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
            let code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
            let html = views::error::error_page(code, "UNSUPPORTED MEDIA TYPE", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
            let code = StatusCode::METHOD_NOT_ALLOWED;
            let html = views::error::error_page(code, "METHOD NOT ALLOWED", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else {
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, "RESOURCE NOT FOUND", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        }
    }

    //  else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
    //     tracing::info!("Passing MethodNotAllowed error through!");
    //     Err(err)
    // } else if err.is_not_found() {
    //     tracing::info!("Passing 404 error through!");
    //     Err(err)
    // } else {
    //     // We should have expected this... Just log and say its a 500
    //     tracing::error!("unhandled rejection: {:?}", err);
    //     let code = StatusCode::INTERNAL_SERVER_ERROR;
    //     let html = views::error::error_page(code, "UNHANDLED_ERROR");
    //     Ok(warp::reply::with_status(warp::reply::html(html), code))
    // }
}

const DEFAULT_WORKSPACE_CONTENT: &str = r#"
|
(h1 title)
(h2 description)
(p "Hello from LISP")
|

### Enjoy your new workspace
This is a program that gets ran each time it is viewed. You can write [Bebop](/bebop) to do all sorts of cool things.
You can write in this text area using markdown. You can see an example above that uses the LISP inline. `title` and `description` are special keywords that are defined for each workspace. There are a bunch of HTML functions that you can use. as well to write functions and use typical functional programming patterns.

|(a (concat "/workspace/" id "/edit") "Edit this workspace here!" )|

### Need help?
- [Bebop Spec](/bebop)
- [What is digitheque](/)
"#;

const DEFAULT_PRELUDE_CONTENT: &str = r#"
(concat "Hello from Prelude! This is executed at the beginning of each of your programs :) " (a "/user/prelude" "Edit your global prelude here!"))
"#;

const GLOBAL_PRELUDE: &str = r#"concat

(def [fun]
    (\ [args body] 
        [def (list (head args)) 
        (\ (tail args) body)]))

(fun [h1 children]
    [concat "<h1>" children "</h1>"])

(fun [h2 children]
    [concat "<h2>" children "</h2>"])

(fun [h3 children]
    [concat "<h3>" children "</h3>"])

(fun [h4 children]
    [concat "<h4>" children "</h4>"])

(fun [h5 children]
    [concat "<h5>" children "</h5>"])

(fun [h6 children]
    [concat "<h6>" children "</h6>"])

(fun [blockquote children]
    [concat "<blockquote>" children "</blockquote>"])

(fun [code children]
    [concat "<code>" children "</code>"])

(fun [pre children]
    [concat "<pre>" children "</pre>"])

(fun [p children]
    [concat "<p>" children "</p>"])

(fun [em children]
    [concat "<em>" children "</em>"]) 

(fun [strike children]
    [concat "<s>" children "</s>"]) 

(fun [strong children]
    [concat "<strong>" children "</strong>"])

(fun [li children]
    [concat "<li>" children "</li>"])

(fun [ul children]
    [concat "<ul>" children "</ul>"])

(fun [tasks children]
    [concat "<ul class='tasks'>" children "</ul>"])

(fun [ol children]
    [concat "<ol>" children "</ol>"])

(def [checked] "<input type='checkbox' checked />")

(def [unchecked] "<input type='checkbox' />")

(fun [ol children]
    [concat "<ol>" children "</ol>"])

(fun [img src alt]
    [concat "<img src='" src "' alt='" alt "' />"])
    
(fun [a href children]
    [concat "<a href='" href "'>" children "</a>"])

(fun [a-out href children]
    [concat "<a target='_blank' href='" href "'>" children "</a>"])

(def [hr]
    "<hr/>")

(def [empty]
    "<div></div>")

(fun [color children]
    [concat "<span style='color: " children ";'>◼</span>" children])

(def [true]
    1)
    
(def [false]
    0)

(def [nil] ())

(fun [not n]
    [if (== n 0) [1] [0]])

(fun [is-nil n] 
    [== n nil])

(fun [not-nil n] 
    [not (== n nil)])

(fun [dec n] [- n 1])

(def [fun] 
    (\ [args body] 
        [def (list (head args)) 
        (\ (tail args) body)]))

(fun [cons x xs]
    [join
        (if (== x [])
            [x]
            [list x])
        xs])

(fun [is-empty l] 
    [if (== l []) 
        [true] 
        [false]])

(fun [len l] 
    [if (is-empty l) 
        [0] 
        [+ 1 (len (tail l))]])

(fun [rec target base step]
    [if (== 0 target)
        [base]
        [step (dec target)
            (\ [] [rec (dec target) base step])]])

(fun [rec-list target base step]
    [if (== 0 (len target))
        [base]
        [step 
            (head target)
            (\ [] [rec-list (tail target) base step])]])

(fun [map target mapper]
    [rec-list target [] (\ [e es] [cons (mapper e) (es)])])

(fun [filter target filterer]
    [rec-list target [] (\ [e es] [if (filterer e) [cons e (es)] [(es)]])])

(fun [nth n l]
    [head (rec n
        l
        (\ [n-1 nthn-1] [tail (nthn-1)]))])

(fun [append n] [eval (cons concat n)])
"#;