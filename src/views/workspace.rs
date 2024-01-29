use html_to_string_macro::html;
use std::fmt::{self, Display};

use super::{Body, Document, Head};
use crate::{
    models,
    views::common::{Footer, Header},
};

pub struct Workspace {
    workspace: models::workspace::WorkspaceWithChildren,
}

impl Display for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main class="split">
                    <section class="">
                        <h1 class="workspace-name">{self.workspace.workspace.name.clone()}</h1>
                        <h2 class="workspace-description">{self.workspace.workspace.description.clone()}</h2>
                        <a href={format!("/workspace/{}/edit", self.workspace.workspace.id)}>"Edit me!"</a>
                        <div class="markdown">
                            {handle_workspace_type(&self.workspace.workspace)}
                        </div>
                    </section>
                    {WorkspaceChildren(self.workspace.children.clone())}
                    <h3>"Add new"</h3>
                    <form action={format!("/workspace/{}", self.workspace.workspace.id)} method="POST">
                        <label>
                            <span>"Name"</span>
                            <input type="text" name="name" required max=64 />
                        </label>
                        <label>
                            <span>"Description"</span>
                            <input type="text" name="description" required max=248 />
                        </label>
                        <input type="hidden" name="type_id" value=2 />

                        <button type="submit">"Add new"</button>
                    </form>
                </main>
            }
        )
    }
}

pub struct WorkspaceChildren(Vec<models::workspace::Workspace>);

impl Display for WorkspaceChildren {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let childs = self.0.iter().map(|workspace| {
            html! {
                <article>
                    <h4>
                        <a href={format!("/workspace/{}", workspace.id)}>{workspace.name.clone()}</a>
                    </h4>
                </article>
            }
        }).collect::<String>();

        write!(
            f,
            "{}",
            html! {
                {childs}
            }
        )
    }
}

pub struct WorkspaceEdit {
    workspace: models::workspace::WorkspaceWithChildren,
}

impl Display for WorkspaceEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            html! {
                <main>
                    <section>
                        <h1 class="workspace-name">{self.workspace.workspace.name.clone()}</h1>
                        <h2 class="workspace-description">{self.workspace.workspace.description.clone()}</h2>
                        <h3>"Edit"</h3>
                        {handle_workspace_edit(&self.workspace.workspace)}
                    </section>
                </main>
            }
        )
    }
}

fn handle_workspace_type(workspace: &models::workspace::Workspace) -> String {
    match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
        _ => {
            let input = workspace.content.clone().unwrap_or(String::from(""));
            println!("{:?}", input);
            let md = bebop_lang::markdown::parser::parse_markdown(&input).unwrap_or(("", vec![]));
            let mut env = bebop_lang::lisp::env::init_env();
            println!("{:?}", md);
            let lisp: String = md.1
                .into_iter()
                .map(bebop_lang::markdown::lisp::markdown_to_lisp)
                .collect();
            

            let input = &format!(r#"concat

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
            
            {}
            "#, lisp);
            
            println!("{}", input);

            let v = bebop_lang::lisp::lisp(&mut env, input);
            v
        }
    }
}

fn handle_workspace_edit(workspace: &models::workspace::Workspace) -> String {
    html! {
        <form action={format!("/workspace/{}/edit", workspace.id)} method="POST">
            <input type="hidden" name="name" value={workspace.name.clone()} />
            <input type="hidden" name="description" value={workspace.description.clone()} />
            {match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
                _ => html! {
                    <textarea name="content">{workspace.content.clone().unwrap_or(String::from("# Edit me to get started!\nMake sure to save using the button at the bottom.\n"))}</textarea>
                },
            }}
            <button type="submit">"Submit"</button>
        </form>
    }
}

pub fn workspace_page(
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> String {
    let header = Header {
        expanded_user: Some(expanded_user),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(Workspace { workspace }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}

pub fn edit_workspace_page(
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> String {
    let header = Header {
        expanded_user: Some(expanded_user),
    };
    let mut body = Body(vec![]);

    body.0.push(Box::new(header));
    body.0.push(Box::new(WorkspaceEdit { workspace }));
    body.0.push(Box::new(Footer));

    let html = Document {
        head: &Head,
        body: &body,
    };
    format!("{}", html)
}
