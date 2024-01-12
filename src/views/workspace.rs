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
        models::workspace::WorkspaceType::Markdown => {
            let input = workspace.content.clone().unwrap_or(String::from(""));
            println!("{:?}", input);
            let md = bebop_lang::markdown::parser::parse_markdown(&input).unwrap_or(("", vec![]));
            let mut env = bebop_lang::lisp::env::init_env();
            println!("{:?}", md);
            let lisp: String = md.1
                .into_iter()
                .map(bebop_lang::markdown::lisp::markdown_to_lisp)
                .collect();
            

            let input = &format!("concat

            (def [fun]
                (\\ [args body] 
                    [def (list (head args)) 
                    (\\ (tail args) body)]))
            
            (fun [h1 children]
                [concat \"<h1>\" children \"</h1>\"])
            
            (fun [h2 children]
                [concat \"<h2>\" children \"</h2>\"])
            
            (fun [h3 children]
                [concat \"<h3>\" children \"</h3>\"])
            
            (fun [h4 children]
                [concat \"<h4>\" children \"</h4>\"])
            
            (fun [h5 children]
                [concat \"<h5>\" children \"</h5>\"])
            
            (fun [h6 children]
                [concat \"<h6>\" children \"</h6>\"])
            
            (fun [code children]
                [concat \"<code>\" children \"</code>\"])
            
            (fun [pre children]
                [concat \"<pre>\" children \"</pre>\"])
            
            (fun [p children]
                [concat \"<p>\" children \"</p>\"])
            
            (fun [i children]
                [concat \"<i>\" children \"</i>\"]) 
            
            (fun [b children]
                [concat \"<b>\" children \"</b>\"])
            
            (fun [li children]
                [concat \"<li>\" children \"</li>\"])
            
            (fun [ul children]
                [concat \"<ul>\" children \"</ul>\"])
            
            (fun [ol children]
                [concat \"<ol>\" children \"</ol>\"])
            
            (fun [img src alt]
                [concat \"<img src='\" src \"' alt='\" alt \"' />\"])
                
            (fun [a href children]
                [concat \"<a href='\" href \"'>\" children \"</a>\"])
            
            (def [hr]
                \"<hr/>\")
            
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
                (\\ [args body] 
                    [def (list (head args)) 
                    (\\ (tail args) body)]))
            
            (fun [cons x xs]
                [join
                    (if (== x [])
                        [x]
                        [list x])
                    xs])
            
            (fun [empty l] 
                [if (== l []) 
                    [true] 
                    [false]])
            
            (fun [len l] 
                [if (empty l) 
                    [0] 
                    [+ 1 (len (tail l))]])
            
            (fun [rec target base step]
                [if (== 0 target)
                    [base]
                    [step (dec target)
                        (\\ [] [rec (dec target) base step])]])
            
            (fun [rec-list target base step]
                [if (== 0 (len target))
                    [base]
                    [step 
                        (head target)
                        (\\ [] [rec-list (tail target) base step])]])
            
            (fun [map target mapper]
                [rec-list target [] (\\ [e es] [cons (mapper e) (es)])])
            
            (fun [filter target filterer]
                [rec-list target [] (\\ [e es] [if (filterer e) [cons e (es)] [(es)]])])

            {}
            ", lisp);
            
            println!("{}", input);

            let v = bebop_lang::lisp::lisp(&mut env, input);
            v
        }
        models::workspace::WorkspaceType::Root => html! {
            <span>"How did this get here??"</span>
        },
    }
}

fn handle_workspace_edit(workspace: &models::workspace::Workspace) -> String {
    html! {
        <form action={format!("/workspace/{}/edit", workspace.id)} method="POST">
            <input type="hidden" name="name" value={workspace.name.clone()} />
            <input type="hidden" name="description" value={workspace.description.clone()} />
            {match models::workspace::WorkspaceType::from_i32(workspace.type_id) {
                models::workspace::WorkspaceType::Markdown => html! {
                    <textarea name="content">{workspace.content.clone().unwrap_or(String::from("Missing data!"))}</textarea>
                },
                models::workspace::WorkspaceType::Root => html! {
                    <span>"How did this get here??"</span>
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
