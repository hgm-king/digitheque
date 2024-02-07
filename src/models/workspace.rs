use crate::{schema::workspace, utils::now, DEFAULT_WORKSPACE_CONTENT};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

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
}

pub struct WorkspaceWithChildren {
    pub workspace: Workspace,
    pub children: Vec<Workspace>,
}

pub struct JoinedWorkspace {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub type_id: i32,
    pub styles: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub content: Option<String>,
    pub parent_id: i32,

    pub child_id: i32,
    pub child_name: String,
    pub child_description: String,
    pub child_type_id: i32,
    pub child_user_id: i32,
    pub child_created_at: NaiveDateTime,
    pub child_updated_at: Option<NaiveDateTime>,
    pub child_deleted_at: Option<NaiveDateTime>,
    pub child_content: Option<String>,
    pub child_parent_id: i32,
}

#[derive(Deserialize)]
pub struct NewWorkspaceApi {
    pub name: String,
    pub description: String,
    pub type_id: i32,
}

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
    pub parent_id: i32
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
            parent_id
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<Workspace, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn create(
    conn: &mut PgConnection,
    new_workspace: &NewWorkspace,
) -> Result<Workspace, diesel::result::Error> {
    diesel::insert_into(workspace::table)
        .values(new_workspace)
        .get_result(conn)
}

pub fn read(conn: &mut PgConnection) -> Result<Vec<Workspace>, diesel::result::Error> {
    workspace::table.load::<Workspace>(conn)
}

pub fn read_by_user_and_id(
    conn: &mut PgConnection,
    user_id: i32,
    id: i32,
) -> Result<Option<WorkspaceWithChildren>, diesel::result::Error> {
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
        .map(|res| {
            if res.len() > 0 {
                let workspace_with_child = WorkspaceWithChildren {
                    workspace: res[0].0.clone(),
                    children: vec![],
                };
                let workspace_with_child = res
                    .into_iter()
                    .filter(|(_, workspace)| workspace.is_some())
                    .fold(workspace_with_child, |mut acc, (_, workspace)| {
                        acc.children.push(workspace.unwrap());
                        acc
                    });

                Some(workspace_with_child)
            } else {
                None
            }
        })
}

pub fn read_root_by_user(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Option<WorkspaceWithChildren>, diesel::result::Error> {
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
        .map(|res| {
            if res.len() > 0 {
                let workspace_with_child = WorkspaceWithChildren {
                    workspace: res[0].0.clone(),
                    children: vec![],
                };
                let workspace_with_child = res
                    .into_iter()
                    .filter(|(_, workspace)| workspace.is_some())
                    .fold(workspace_with_child, |mut acc, (_, workspace)| {
                        acc.children.push(workspace.unwrap());
                        acc
                    });

                Some(workspace_with_child)
            } else {
                None
            }
        })
}

pub fn delete(conn: &mut PgConnection, workspace: &Workspace) -> QueryResult<usize> {
    diesel::update(workspace)
        .set((workspace::deleted_at.eq(Some(now())),))
        .execute(conn)
}

pub fn update(conn: &mut PgConnection, id: i32, workspace: EditWorkspaceApi) -> QueryResult<usize> {
    let edit_workspace = EditWorkspace {
        updated_at: Some(now()),
        name: workspace.name,
        description: workspace.description,
        content: workspace.content,
    };

    diesel::update(workspace::table)
        .set(edit_workspace)
        .filter(workspace::id.eq(id))
        // .set((workspace::updated_at.eq(Some(now())),))
        .execute(conn)
}

#[test]
fn test_diesel() {
    let (parent, children) = diesel::alias!(workspace as parent, workspace as children);
    let sql = parent
    .left_join(
        children.on(children
            .field(workspace::parent_id)
            .eq(parent.field(workspace::id))),
    )
    .filter(parent.field(workspace::deleted_at).is_null())
    .filter(children.field(workspace::deleted_at).is_null())
    .filter(parent.field(workspace::user_id).eq(1))
    .filter(children.field(workspace::user_id).eq(1))
    .filter(parent.field(workspace::id).eq(2));
    assert_eq!(
        diesel::debug_query::<diesel::pg::Pg, _>(&sql).to_string(),
        "SELECT `users`.`id`, `users`.`name` FROM `users` \
        WHERE (`users`.`id` = ?) -- binds: [1]"
    );
}
