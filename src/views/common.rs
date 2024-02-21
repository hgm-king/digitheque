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
                                        <li><a href="/login">"Login"</a></li>
                                    },
                                    Some(user) => html! {
                                        <li><a href="/root">{&user.user.username}</a></li>
                                        <li><a href="/logout">"Logout"</a></li>
                                        <li><a href="/feed">"Feed"</a></li>
                                        <li><a href="/prelude">"Prelude"</a></li>
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

pub struct Landing {
    pub expanded_user: Option<ExpandedUser>,
}

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
                        <div class="cta">{
                            if self.expanded_user.is_none() { html! {
                                <a class="button-link" href="/signup">"Get started"</a>
                            } }
                            else  { html! {
                                <a class="button-link" href="/root">"View profile"</a>
                            }
                        } }
                </div>
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

                    <figure class="banner-img">
                        <img class="full" src="/img/dithers/vosges_dithered.png" alt="vosges" />
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
                        <p>"Second, our "<a href="/bebop">"Bebop-lang"</a>" preprocessor helps writers build out dynamic and
                            re-usable elements with a simplified programming interface; enabling custom publications."</p>
                    </div>
                </main>
            }
        )
    }
}

pub fn landing_page(expanded_user: Option<ExpandedUser>) -> String {
    let body = Body(vec![
        Box::new(Header {
            expanded_user: expanded_user.clone(),
        }),
        Box::new(Landing { expanded_user }),
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
                    <div class="markdown-section">
                        <h3>"Markdown Specification"</h3>
                        <p>"Get started by reading the "<a href="https://www.markdownguide.org/getting-started/">"markdown
                                guide"</a>";
                            it touches upon most of the syntax in this language. We use markdown to specify the content in our
                            documents. It is analogous to HTML, with which the reader may be familiar."</p>
                        <h4>"Headings"</h4>
                        <p>"A heading translates to an "<code>"h1"</code>" or differently numbered header in HTML."</p>
                        <p>"To specify an "<code>"h1"</code>", use the <code>#</code> symbol followed by a space and then text. The
                            number of <code>#</code> symbols determines the header level."</p>
                        <p>"Example:"</p>
                        <pre>r#"# Top level H1
## Second level H2
### Third level H3
#### Fourth level H4
##### Fifth level H5
###### Sixth level H6
"#</pre>
                        <h4>"Lists"</h4>
                        <p>"Lists can be either ordered or unordered, both of which correspond to "<code>"ol"</code>" and"
                            <code>"ul"</code>
                            "in HTML respectively. We also provide a task list as well which is not found in HTML."
                        </p>
                        <p>"An ordered list is a series of lines that begin with "<em>"any number"</em>", a "<code>"."</code>", and a
                            space
                            followed by text."</p>
                        <pre>r#"1. the numbers
3. do not matter
1. and will order themselves
4000. correctly when rendered
"#</pre>
                        <p>"An unordered list is a series of lines that begin with a "<code>"-"</code>" and a space followed by text."
                        </p>
                        <pre>r#"- items in
- a ul
- don't lend
- themselves to
- any particular
- order
"#</pre>
                        <p>"A task list is a series of lines that begin with "<code>"- [ ]"</code>" and a space followed by text. To
                            check the box as done, use "<code>"- [x]"</code>"."</p>
                        <pre>r#"- [ ] cookies
- [x] toilet paper
- [ ] beans
- [ ] milk
"#</pre>
                        <h4>"Codeblock"</h4>
                        <p>"A codeblock does not correspond directly to an HTML element, although they are very commonly found on
                            the
                            internet. The idea is that the formatting in the codeblock is preserved when rendering."</p>
                        <p>"Two write a codeblock, use 3 backticks followed by the language of code being used, then any series
                            of
                            lines of text, and then a line with 3 backticks."</p>
                        <p>"Example:"</p>
                        <p><em>"example goes here"</em></p>
                        <h4>"Blockquote"</h4>
                        <p>"A blockquote corresponds directly to the <code>blockquote</code> tag in HTML. It is written by using
                            a"
                            <code>"&gt;"</code>" followed by a space and then text. It ends when the line ends."
                        </p>
                        <p>"Example:"</p>
                        <pre>"&gt; Four score and seven years ago..."</pre>
                        <h4>"Horizontal Rule"</h4>
                        <p>"A horizontal corresponds to the <code>hr</code> tag in HTML. It is written by simply using"
                            <code>"---"</code>" on a line."
                        </p>
                        <p>"Example:"</p>
                        <pre>"---"</pre>
                        <h4>"Inline Elements"</h4>
                        <p>"If none of the above elements are used, then a line with text in it is considered a paragraph. There
                            can
                            be inline elements inside paragraphs."</p>
                        <h5>"Bold, Italic, Inline Code &amp; Strikethrough"</h5>
                        <p>"These elements correspond to their similarly named HTML tags."</p>
                        <p>"Example:"</p>

                        <h5>"Link &amp; Image"</h5>
                        <p>"Both links and images have corresponding tags in HTML. We also provide an external link that will
                            open a
                            new tab when clicked on."</p>
                        <p>"Example:"</p>
                        <pre>r#"[Link text](/bebop)
^[External link](https://google.com)
![Image alt text](https://picsum.photos/200)
"#</pre>
                        <h5>"Color Swatch"</h5>
                        <p>"This is a custom element, that represents a given hex code."</p>
                        <p>"Example:"</p>
                        <pre>"#ff5523 is a cool color yeah?"</pre>
                        <h5>"Plaintext"</h5>
                        <p>"Any text that does not fall under the other inline elements' syntax."</p>
                        <p>"Example:"</p>
                        <pre>"Plain planes find themselves on the plain."</pre>
                    </div>
                    <div class="lisp-section">
                        <h3>"LISP Specification"</h3>
                        <h4>"Grammar"</h4>
                        <pre>r#"Sexp = ( Symbol+ )
Qexp = [ Symbol+ ]
Number = 1234567890
Symbol = _+\\:-*/=|!&amp;%a-zA-Z1234567890
String = Symbol
"#</pre>
                    <h4>"Syntax &amp; Types"</h4>
                    <p>"Our types are simple and cover pretty much all of our bases. I don't believe that we will need to
                        change
                        our design here."</p>
                    <h5>"Number"</h5>
                    <p>"Numbers like we are all familiar with. (ie. "<code>"1"</code>", "<code>"1.1"</code>", "<code>"1.1e+13"</code>", "
                        <code>"1.1e-13"</code>")"
                    </p>
                    <h5>"Symbol"</h5>
                    <p>"Symbols are names that can be assigned to any value. (ie. "<code>"add"</code>", "<code>"def"</code>", "
                        <code>"fun"</code>", "<code>"some-var"</code>")"
                    </p>
                    <p>"Usage: "<code>"def [symbol-name] value"</code></p>
                    <h5>"String"</h5>
                    <p>"Strings are characters delimited by double quotes. (ie. "<code>"\"c'ect ci nest pa un pipe?\""</code>", "
                        <code>"\"hg king\""</code>")"
                    </p>
                    <h5>"S-Expression"</h5>
                    <p>"S-Expressions are used to call and evaluate functions. (ie." <code>"(+ 1 2 3)"</code>", "
                        <code>"(- (+ 9 1) (* 5 2))"</code>", "<code>"(list 1 2 3 4)"</code>", "<code>"(== [] [])"</code>")"
                    </p>
                    <p>"Usage: "<code>"(function arg0 arg1 arg2)"</code></p>
                    <h5>"Q-Expression"</h5>
                    <p>"Q-Expressions are lists of values, remains unevaluated. (ie. "<code>"[1 1 1 1]"</code>", "
                        <code>"[+ 9 (== [] [])]"</code>")"
                    </p>
                    <p>"Usage: "<code>"[elem0 elem1 elem2]"</code></p>
                    <h5>"Lambda"</h5>
                    <p>"Lambda functions are how you build functions, can be partially applied. (ie."
                        <code>"(\\ [a b] [+ a b])"</code>")"
                    </p>
                    <p>"Usage: "<code>"(\\ [arg-list] [body])"</code></p>
                    <h4>"Predefined Functions"</h4>
                    <p>"When writing your Bebop programs, we provide a set of predefined functions that will come in handy. They are most of the FP method for list processing that you would expect. Read them below:"</p>
                    <pre>r#"concat

(def [fun]
    (\ [args body] 
        [def (list (head args)) 
        (\ (tail args) body)]))

(fun [h1 children]
    [concat "&lt;h1&gt;" children "&lt;/h1&gt;"])

(fun [h2 children]
    [concat "&lt;h2&gt;" children "&lt;/h2&gt;"])

(fun [h3 children]
    [concat "&lt;h3&gt;" children "&lt;/h3&gt;"])

(fun [h4 children]
    [concat "&lt;h4&gt;" children "&lt;/h4&gt;"])

(fun [h5 children]
    [concat "&lt;h5&gt;" children "&lt;/h5&gt;"])

(fun [h6 children]
    [concat "&lt;h6&gt;" children "&lt;/h6&gt;"])

(fun [blockquote children]
    [concat "&lt;blockquote&gt;" children "&lt;/blockquote&gt;"])

(fun [code children]
    [concat "&lt;code&gt;" children "&lt;/code&gt;"])

(fun [pre children]
    [concat "&lt;pre&gt;" children "&lt;/pre&gt;"])

(fun [p children]
    [concat "&lt;p&gt;" children "&lt;/p&gt;"])

(fun [em children]
    [concat "&lt;em&gt;" children "&lt;/em&gt;"]) 

(fun [strike children]
    [concat "&lt;s&gt;" children "&lt;/s&gt;"]) 

(fun [strong children]
    [concat "&lt;strong&gt;" children "&lt;/strong&gt;"])

(fun [li children]
    [concat "&lt;li&gt;" children "&lt;/li&gt;"])

(fun [ul children]
    [concat "&lt;ul&gt;" children "&lt;/ul&gt;"])

(fun [tasks children]
    [concat "&lt;ul class='tasks'&gt;" children "&lt;/ul&gt;"])

(fun [ol children]
    [concat "&lt;ol&gt;" children "&lt;/ol&gt;"])

(def [checked] "&lt;input type='checkbox' checked /&gt;")

(def [unchecked] "&lt;input type='checkbox' /&gt;")

(fun [ol children]
    [concat "&lt;ol&gt;" children "&lt;/ol&gt;"])

(fun [img src alt]
    [concat "&lt;img src='" src "' alt='" alt "' /&gt;"])
    
(fun [a href children]
    [concat "&lt;a href='" href "'&gt;" children "&lt;/a&gt;"])

(fun [a-out href children]
    [concat "&lt;a target='_blank' href='" href "'&gt;" children "&lt;/a&gt;"])

(def [hr]
    "&lt;hr/&gt;")

(def [empty]
    "&lt;div&gt;&lt;/div&gt;")

(fun [color children]
    [concat "&lt;span style='color: " children ";'&gt;◼&lt;/span&gt;" children])

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

(fun [append n] [eval (cons concat n)])"#</pre>
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
