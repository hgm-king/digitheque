use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};

pub struct Signup;

impl Display for Signup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <section class="signup single-form">
                    <h4>"Signup Form"</h4>
                    <form action="/user/signup" method="POST">
                        <fieldset class="signup-fields">
                            <legend>"User Credentials"</legend>
                            <a href="/user/login">"Have an account?"</a>
                            <label>
                                <span>"Username:"</span>
                                <input type="text" name="username" required max=48 />
                            </label>
                            <label>
                                <span>"Password:"</span>
                                <input type="password" name="password" max=48 />
                            </label>
                            <label>
                                <span>"Confirm Password:"</span>
                                <input type="password" name="confirm_password" max=48 />
                            </label>
                            <div class="error">"error"</div>
                            <button type="submit">"Signup"</button>
                        </fieldset>
                    </form>
                </section>
            }
        )
    }
}

pub struct Login;

impl Display for Login {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <section class="login single-form">
                    <h4>"Login Form"</h4>
                    <form action="/user/login" method="POST">
                        <fieldset class="login-fields">
                            <legend>"User Credentials"</legend>
                            <a href="/user/signup">"Need an account?"</a>
                            <label>
                                <span>"Username:"</span>
                                <input type="text" name="username" required max=48 />
                            </label>
                            <label>
                                <span>"Password:"</span>
                                <input type="password" name="password" max=48 />
                            </label>
                            <div class="error">"error"</div>
                            <button type="submit">"Login"</button>
                        </fieldset>
                    </form>
                </section>
            }
        )
    }
}


pub fn login_form() -> String {
    let body = Body(vec![Login]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    html.to_string()
}

pub fn signup_form() -> String {
    let body = Body(vec![Signup]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    html.to_string()
}