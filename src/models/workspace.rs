use crate::{
    models::user::User,
    schema::workspace,
    utils::{now, sanitize_html},
    DEFAULT_WORKSPACE_CONTENT, DOMAIN,
};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use html_to_string_macro::html;
// use rss::{Channel, ChannelBuilder};
use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(PartialEq)]
pub enum WorkspaceType {
    Root = 1,
    Markdown = 2,
}

impl WorkspaceType {
    pub fn from_i32(value: i32) -> WorkspaceType {
        match value {
            1 => WorkspaceType::Root,
            2 => WorkspaceType::Markdown,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

impl Display for WorkspaceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkspaceType::Root => 1,
                WorkspaceType::Markdown => 2,
            }
        )
    }
}

#[derive(Clone, Debug, Identifiable, Selectable, Queryable, AsChangeset)]
#[diesel(belongs_to(models::workspace_element::WorkspaceElement))]
#[diesel(table_name = workspace)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub type_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub content: Option<String>,
    pub parent_id: i32,
    pub is_published: bool,
}

impl Workspace {
    pub fn new(
        conn: &mut PgConnection,
        new_workspace: &NewWorkspace,
    ) -> Result<Workspace, diesel::result::Error> {
        diesel::insert_into(workspace::table)
            .values(new_workspace)
            .get_result(conn)
    }

    pub fn details(&self) -> String {
        html! {
            <dl id="workspace-details">
                <dt>"Name"</dt>
                <dd>{self.name.clone()}</dd>
                <dt>"Description"</dt>
                <dd>{self.description.clone()}</dd>
                <dt>"Status"</dt>
                <dd>{if self.is_published == true {
                    "published"
                } else {
                    "draft"
                }}</dd>
            </dl>
        }
    }

    pub fn actions(&self, is_editing: bool) -> String {
        html! {
            <ul id="workspace-actions">
                <li>
                    {if self.is_root() {
                        String::from("Root workspace")
                    } else {
                        self.link_to_parent()
                    }}
                </li>
                <li>
                    {if is_editing == true {
                        html! { <a href={format!("/workspace/{}", self.id)}>"✕ Cancel edit"</a> }
                    } else {
                        self.link_to_edit()
                    }}
                </li>
                {
                    if self.is_root() {
                        html! {
                            <li>{User::link_to_prelude()}</li>
                        }
                    } else {
                        html! {
                            <li>{self.publish_form()}</li>
                        }
                    }
                }
                <li>
                    <details>
                        <summary>"Add subworkspace"</summary>
                        {self.new_workspace_form()}
                    </details>
                </li>
            </ul>
        }
    }

    pub fn link_to_self(&self) -> String {
        html! {
            <a href={format!("/workspace/{}", self.id)}>{self.name.clone()}</a>
        }
    }

    pub fn link_to_parent(&self) -> String {
        html! {<a class="red" href={format!("/workspace/{}", self.parent_id)}>"← Back to parent"</a>}
    }

    // how do we get the API type in here...
    pub fn new_workspace_form(&self) -> String {
        html! {
            <form action={format!("/workspace/{}/new", self.id)} method="POST">
                <label>
                    <span>"Name"</span>
                    <input type="text" name="name" required max=64 />
                </label>
                <label>
                    <span>"Description"</span>
                    <input type="text" name="description" required max=248 />
                </label>
                <input type="hidden" name="type_id" value={WorkspaceType::Markdown} />
                <button type="submit">"Add workspace"</button>
            </form>
        }
    }

    pub fn is_root(&self) -> bool {
        WorkspaceType::from_i32(self.type_id) == WorkspaceType::Root
    }

    pub fn publish_form(&self) -> String {
        html! {
            <form action={format!("/workspace/{}/publish", self.id)} method="POST" id="publish-workspace">
                <input type="hidden" name="is_published" value={!self.is_published} />
                <button type="submit" class="submit-publish">
                    {
                        if self.is_published == false { "● Publish" }
                        else { "○ Unpublish" }
                    }
                </button>
            </form>
        }
    }

    pub fn edit_self_form(&self) -> String {
        html! {
            <form action={format!("/workspace/{}", self.id)} method="POST">
                <label>
                    <span>"Name"</span>
                    <input type="text" name="name" value={self.name.clone()} required max=64 />
                </label>
                <label>
                    <span>"Description"</span>
                    <input type="text" name="description" value={self.description.clone()} required max=248 />
                </label>
                <textarea name="content">{self.content.clone().unwrap_or(String::from("# Edit me to get started!\nMake sure to save using the button at the bottom.\n"))}</textarea>
                <button type="submit">"Submit"</button>
            </form>
        }
    }

    // how do we get these links to match the routes??
    pub fn link_to_edit(&self) -> String {
        html! {
            <a href={format!("/workspace/{}/edit", self.id)}>"✎ Start drafting"</a>
        }
    }

    pub fn execute_content(&self, prelude: String) -> String {
        let input = self.content.clone().unwrap_or(String::from(""));

        // parse markdown doc
        let (_, md) = bebop_lang::markdown::parser::parse_markdown(&input).unwrap_or(("", vec![]));
        let mut env = bebop_lang::lisp::env::init_env();

        // convert each line into lisp
        let lisp: String = md
            .into_iter()
            .map(bebop_lang::markdown::lisp::markdown_to_lisp)
            .collect();

        // build up program
        let input = &format!(
            r#"{}
{}
{}
"#,
            prelude,
            self.get_lisp_values(),
            lisp
        );

        tracing::info!("{}", input);

        // execute
        let v = bebop_lang::lisp::lisp(&mut env, input);
        v
    }

    pub fn get_lisp_values(&self) -> String {
        format!(
            r#"(def [title] "{}")
(def [description] "{}")
(def [updated-at] "{}")
(def [id] "{}")
"#,
            self.name,
            self.description,
            self.updated_at.unwrap_or_default(),
            self.id,
        )
    }

    pub fn delete(&self, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::update(self)
            .set((workspace::deleted_at.eq(Some(now())),))
            .execute(conn)
    }

    pub fn to_rss_item(&self, author: String) -> rss::Item {
        rss::ItemBuilder::default()
            .title(Some(self.name.clone()))
            .description(Some(self.description.clone()))
            .link(Some(format!("{}/{}/workspace/{}", DOMAIN, author, self.id)))
            .guid(Some(rss::Guid {
                value: format!("{}/{}/workspace/{}", DOMAIN, author, self.id),
                permalink: true,
            }))
            .pub_date(Some(
                self.updated_at.unwrap_or_default().and_utc().to_rfc2822(),
            ))
            .build()
    }
}

#[derive(Clone)]
pub struct WorkspaceWithChildren {
    pub workspace: Workspace,
    pub children: Vec<Workspace>,
}

impl WorkspaceWithChildren {
    pub fn subworkspaces(&self) -> String {
        let childs = if self.children.len() == 0 {
            html! {
                <li>"No Subworkspaces yet"</li>
            }
        } else {
            self.children
                .iter()
                .map(|workspace| {
                    html! {
                        <li>
                            {workspace.link_to_self()}
                        </li>
                    }
                })
                .collect::<String>()
        };

        html! {
            <ul>
                {childs}
            </ul>
        }
    }

    pub fn from_joined(joined: Vec<(Workspace, Option<Workspace>)>) -> Option<Self> {
        if joined.len() <= 0 {
            return None;
        };

        let workspace_with_child = WorkspaceWithChildren {
            workspace: joined[0].0.clone(),
            children: vec![],
        };
        let workspace_with_child = joined
            .into_iter()
            .filter(|(_, workspace)| workspace.is_some())
            .fold(workspace_with_child, |mut acc, (_, workspace)| {
                acc.children.push(workspace.unwrap());
                acc
            });

        Some(workspace_with_child)
    }

    pub fn read_by_user_and_id(
        conn: &mut PgConnection,
        user_id: i32,
        id: i32,
    ) -> Result<Option<Self>, diesel::result::Error> {
        let (parent, children) = diesel::alias!(workspace as parent, workspace as children);
        parent
            .left_join(
                children.on(children
                    .field(workspace::parent_id)
                    .eq(parent.field(workspace::id))),
            )
            .filter(parent.field(workspace::deleted_at).is_null())
            .filter(children.field(workspace::deleted_at).is_null())
            .filter(parent.field(workspace::user_id).eq(user_id))
            // .filter(children.field(workspace::user_id).eq(user_id)) or null?
            .filter(parent.field(workspace::id).eq(id))
            .load::<(Workspace, Option<Workspace>)>(conn)
            .map(|res| Self::from_joined(res))
    }

    pub fn read_root_by_user(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<Option<Self>, diesel::result::Error> {
        // workspace::table.first::<Workspace>(conn)
        let (parent, children) = diesel::alias!(workspace as parent, workspace as children);
        parent
            .left_join(
                children.on(children
                    .field(workspace::parent_id)
                    .eq(parent.field(workspace::id))),
            )
            .filter(parent.field(workspace::deleted_at).is_null())
            .filter(children.field(workspace::deleted_at).is_null())
            .filter(parent.field(workspace::user_id).eq(user_id))
            // .filter(children.field(workspace::user_id).eq(user_id)) or null?
            .filter(parent.field(workspace::parent_id).eq(-1))
            .load::<(Workspace, Option<Workspace>)>(conn)
            .map(|res| Self::from_joined(res))
    }
}

#[derive(Deserialize)]
pub struct NewWorkspaceApi {
    pub name: String,
    pub description: String,
    pub type_id: i32,
}

//
// Edit workspace
//
#[derive(Deserialize)]
pub struct EditWorkspaceApi {
    pub name: String,
    pub description: String,
    pub content: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = workspace)]
pub struct EditWorkspace {
    pub name: String,
    pub description: String,
    pub content: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<EditWorkspaceApi> for EditWorkspace {
    fn from(ws: EditWorkspaceApi) -> Self {
        EditWorkspace {
            name: sanitize_html(&ws.name),
            description: sanitize_html(&ws.description),
            content: Some(sanitize_html(&ws.content.unwrap_or(String::from("")))),
            updated_at: Some(now()),
        }
    }
}

impl EditWorkspaceApi {
    pub fn update(self, conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::update(workspace::table)
            .set::<EditWorkspace>(self.into())
            .filter(workspace::id.eq(id))
            .execute(conn)
    }
}

//
// Publish Workspace
//
#[derive(Deserialize)]
pub struct PublishWorkspaceApi {
    pub is_published: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = workspace)]
pub struct PublishWorkspace {
    pub is_published: bool,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<PublishWorkspaceApi> for PublishWorkspace {
    fn from(ws: PublishWorkspaceApi) -> Self {
        PublishWorkspace {
            is_published: ws.is_published,
            updated_at: Some(now()),
        }
    }
}

impl PublishWorkspaceApi {
    pub fn publish(self, conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::update(workspace::table)
            .set::<PublishWorkspace>(self.into())
            .filter(workspace::id.eq(id))
            .execute(conn)
    }
}

#[derive(Insertable)]
#[diesel(table_name = workspace)]
pub struct NewWorkspace {
    pub name: String,
    pub description: String,
    pub type_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub content: Option<String>,
    pub parent_id: i32,
    pub is_published: bool,
}

impl NewWorkspace {
    pub fn new(new_workspace: NewWorkspaceApi, creator_user_id: i32, parent_id: i32) -> Self {
        NewWorkspace {
            user_id: creator_user_id,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            name: new_workspace.name,
            description: new_workspace.description,
            type_id: new_workspace.type_id,
            content: Some(String::from(DEFAULT_WORKSPACE_CONTENT)),
            parent_id,
            is_published: false,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<Workspace, diesel::result::Error> {
        Workspace::new(conn, self)
    }
}
