use crate::{schema::workspace_element, utils::now};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Identifiable, Selectable, Queryable, AsChangeset)]
#[diesel(belongs_to(Workspace))]
#[diesel(table_name = workspace_element)]
pub struct WorkspaceElement {
    pub id: i32,
    pub parent_id: i32,
    pub child_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = workspace_element)]
pub struct NewWorkspaceElement {
    pub parent_id: i32,
    pub child_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl NewWorkspaceElement {
    pub fn new(parent_id: i32, child_id: i32, creator_user_id: i32) -> Self {
        NewWorkspaceElement {
            user_id: creator_user_id,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            parent_id,
            child_id,
        }
    }

    pub fn insert(
        &self,
        conn: &mut PgConnection,
    ) -> Result<WorkspaceElement, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn create(
    conn: &mut PgConnection,
    new_workspace_element: &NewWorkspaceElement,
) -> Result<WorkspaceElement, diesel::result::Error> {
    diesel::insert_into(workspace_element::table)
        .values(new_workspace_element)
        .get_result(conn)
}
