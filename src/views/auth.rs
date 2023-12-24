use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{
    common::{Footer, Header},
    Body, Document, Head,
};

pub struct Signup {
    error: String,
}

impl Display for Signup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <header>
                        <h1>"Signup form"</h1>
                    </header>
                    <form id="signup" action="/user/signup" method="POST">
                        <fieldset class="signup-fields">
                            <legend>"User credentials"</legend>
                            <a href="/user/login">"Have an account?"</a>
                            <label>
                                <span>"Username"</span>
                                <input type="text" name="username" required max=48 />
                            </label>
                            <label>
                                <span>"Password"</span>
                                <input type="password" name="password" max=48 />
                            </label>
                            <label>
                                <span>"Confirm Password"</span>
                                <input type="password" name="confirm_password" max=48 />
                            </label>
                            <div class="error">{self.error.clone()}</div>
                            <button type="submit">"Signup"</button>
                        </fieldset>
                    </form>
                </main>
            }
        )
    }
}

pub struct Login {
    error: String,
}

impl Display for Login {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <header>
                        <h1>"Login form"</h1>
                    </header>
                    <form id="login" action="/user/login" method="POST">
                        <fieldset class="login-fields">
                            <legend>"User Credentials"</legend>
                            <a href="/user/signup">"Need an account?"</a>
                            <label>
                                <span>"Username"</span>
                                <input type="text" name="username" required max=48 />
                            </label>
                            <label>
                                <span>"Password"</span>
                                <input type="password" name="password" max=48 />
                            </label>
                            <div class="error">{self.error.clone()}</div>
                            <button type="submit">"Login"</button>
                        </fieldset>
                    </form>
                </main>
            }
        )
    }
}

pub fn login_form(error: Option<String>) -> String {
    let body = Body(vec![
        Box::new(Header {
            expanded_user: None
        }),
        Box::new(Login {
            error: error.unwrap_or(String::from("")),
        }),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    html.to_string()
}

pub fn signup_form<'a>(error: Option<String>) -> String {
    let body: Body = Body(vec![
        Box::new(Header {
            expanded_user: None
        }),
        Box::new(Signup {
            error: error.unwrap_or(String::from("")),
        }),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    html.to_string()
}
