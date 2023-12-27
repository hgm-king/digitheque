use crate::{
    models,
    views::workspace,
};

pub fn profile_page(expanded_user: models::user::ExpandedUser, workspace: models::workspace::WorkspaceWithChildren,) -> String {
    workspace::workspace_page(expanded_user, workspace)
}
