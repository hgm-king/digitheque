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
                <main id="signup">
                    <h1>"Signup form"</h1>
                    <form action="/signup" method="POST">
                        <fieldset class="signup-fields">
                            <legend>"User credentials"</legend>
                            <div>
                                <a href="/login">"Have an account?"</a>
                            </div>

                            <div>
                                <label>
                                    <span>"Username"</span>
                                    <input type="text" name="username" required max=48 pattern="[a-zA-Z0-9]+" />
                                </label>
                                <small>"Use only letters and numbers, please"</small>
                            </div>
                            <div>
                                <label>
                                    <span>"Password"</span>
                                    <input type="password" name="password" required max=48 />
                                </label>
                            </div>
                            <div>
                                <label>
                                    <span>"Confirm Password"</span>
                                    <input type="password" name="confirm_password" required max=48 />
                                </label>
                            </div>
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
                <main id="login">
                    <h1>"Login form"</h1>
                    <form action="/login" method="POST">
                        <fieldset class="login-fields">
                            <legend>"User credentials"</legend>
                            <div>
                                <a href="/signup">"Need an account?"</a>
                            </div>
                            <div>
                                <label>
                                    <span>"Username"</span>
                                    <input type="text" name="username" required max=48 />
                                </label>
                            </div>
                            <div>
                                <label>
                                    <span>"Password"</span>
                                    <input type="password" name="password" required max=48 />
                                </label>
                            </div>
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
            expanded_user: None,
        }),
        Box::new(Login {
            error: error.unwrap_or(String::from("")),
        }),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head {
            title: "Login".to_string(),
            description: "Login to Digitheque".to_string()
        },
        body: &body,
    };
    html.to_string()
}

pub fn signup_form<'a>(error: Option<String>) -> String {
    let body: Body = Body(vec![
        Box::new(Header {
            expanded_user: None,
        }),
        Box::new(Signup {
            error: error.unwrap_or(String::from("")),
        }),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head {
            title: "Signup".to_string(),
            description: "Signup to Digitheque".to_string()
        },
        body: &body,
    };
    html.to_string()
}
