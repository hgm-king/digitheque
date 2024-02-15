use html_to_string_macro::html;
use std::fmt::{self, Display};

use crate::models::user::ExpandedUser;

use super::{Body, Document, Head};

pub struct Header {
    pub expanded_user: Option<ExpandedUser>,
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <header id="banner">
                    <span id="logo"><a href="/" title="Digitheque.io">"Digitheque"</a></span>
                    <nav>
                        <ul>
                            {
                                match &self.expanded_user {
                                    None => html! {
                                        <li><a href="/user/login">"Login"</a></li>
                                    },
                                    Some(user) => html! {
                                        <li><a href="/user">{&user.user.username}</a></li>
                                        <li><a href="/user/logout">"Logout"</a></li>
                                    }
                                }
                            }
                            <li><a href="/">"About"</a></li>
                        </ul>
                    </nav>
                </header>
            }
        )
    }
}

pub struct Footer;

impl Display for Footer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <footer>
                    <div>"Copyright Digitheque Ltd. 2023"</div>
                    <div><a href="https://github.com/hgm-king/digitheque">"Code"</a></div>
                    <div>"Built by "<a href="mailto:hgmaxwellking@gmail.com">"HG King"</a></div>
                </footer>
            }
        )
    }
}

pub struct Landing;

impl Display for Landing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main id="index">
                    <div class="title">
                        <h1>"Digitheque: Online Publishing!"</h1>
                        <span class="sub-title">"Draft and publish your custom"<br />"magazines, pamphlets, and notes."</span>
                        <div class="cta"><a class="button-link" href="/user/signup">"Get started"</a></div>
                    </div>

                    <p class="paragraph-1">"The Digitheque is a place where people can create customized outlets and post
                        them to
                        the wider internet.
                        We believe the web can be a fun place where everyone has the right to be
                        their own publisher. There is no better time than now to get started."</p>
                    <div class="paragraph-2">
                        <p>"Without a centralized corporation governing how people consume
                            media, people are free to gravitate towards whatever information they choose.
                            No algorithmic manipulation."
                        <br />"Keep reading to learn how we do it!"</p>
                    </div>

                    <figure class="banner-img full">
                        <img src="/img/dithers/vosges_dithered.png" alt="vosges" />
                    </figure>
                    <div class="paragraph-3">
                        <p>"The key concept of Digitheque is the <em>workspace</em>. These can
                            represent a notepad, magazine article, pamphlet, or even a flier. Inside a workspace, you can write
                            whatever your heart desires. When you feel fit, you are able to
                            publish them to your <em>feed</em> for your subscribers to read."</p>
                        <p>"This way, you are acting as your own publisher that broadcasts out to the world without any
                            interference."
                        </p>
                    </div>
                    <div class="paragraph-4">
                        <p>"In order to
                            achieve this, we use a handful of simple and established web technologies:"</p>
                        <ul>
                            <li><strong>"Markdown"</strong>" to write your
                                articles or notes"</li>
                            <li><strong>"CSS"</strong>" to build up your personal style"</li>
                            <li><strong>"RSS"</strong>" to publish to the world"</li>
                        </ul>
                        <p>"These tools are all that we need to hook into the natural network effect of the internet that social
                            media
                            tries to replicate."
                        </p>
                    </div>
                    <div class="paragraph-5">
                        <p>"The Digitheque ties this all together by offering two key features."</p>
                        <p>"First, workspaces are nestable; allowing for any type of structure that fits the need of
                            the publication."
                        </p>
                        <p>"Second, our "<a href="/bebop.html">"Bebop-lang"</a>" preprocessor helps writers build out dynamic and
                            re-usable elements with a simplified programming interface; enabling custom publications."</p>
                    </div>
                </main>
            }
        )
    }
}

pub fn landing_page(expanded_user: Option<ExpandedUser>) -> String {
    let body = Body(vec![
        Box::new(Header { expanded_user }),
        Box::new(Landing),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}

pub struct Bebop;

impl Display for Bebop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main id="bebop">
                    <div class="title">
                        <h1>"Bebop Programming"</h1>
                        <span>"LISP based Markdown preprocessor"</span>
                    </div>
                    <img class="img-1" src="/img/dithers/pads_dithered.png" />
                    <div class="paragraph-1">
                        <h2>"About"</h2>
                        <p>"Bebop is so named to allow people to freely express themselves. We want to let people
                            write better
                            documents through simple bebop programs. The basis of the language is Markdown, a simplified version
                            of
                            HTML. Writers can quickly pick up the syntax and draft very sophisticated documents from the start."
                        </p>
                    </div>
                    <img class="img-2" src="/img/dithers/fountain_dithered.png" />
                    <p class="paragraph-2">"We inject another layer on top of the Markdown with a simple lisp dialect that
                        preprocesses the Markdown
                        document before rendering. This should look familiar to PHP for those in the know. You can use the lisp
                        to define values and functions that can be re-used all throughout the program. In turn, one can write
                        documents that share elements and structure to build a custom framework."</p>
                    <img class="img-3" src="/img/dithers/etretat_dithered.png" />

                    <p class="paragraph-3">"A bebop program looks like a regular markdown document that contains lisp code. All
                        bebop programs output a string of text that represents the document. Programs can use entirely markdown
                        or entirely lisp if the author chooses. The runtime converts all markdown into special lisp functions,
                        so one can imagine
                        the markdown as syntactic sugar for code."
                    </p>
                    <div class="paragraph-4">
                        <p>"After converting all the markdown into lisp, the resulting code
                            will look like a series of statements
                            that must resolve to a string. Finally, the runtime executes
                            the lisp and concatenates each element together to produce the resulting document. We will now define
                            the specs for both the markdown and the LISP side of things."</p>
                        <p>"The specifications for both the Markdown and the LISP follow this section."</p>
                    </div>
                </main>
            }
        )
    }
}

pub fn bebop_page(expanded_user: Option<ExpandedUser>) -> String {
    let body = Body(vec![
        Box::new(Header { expanded_user }),
        Box::new(Bebop),
        Box::new(Footer),
    ]);
    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
