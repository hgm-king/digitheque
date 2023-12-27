use crate::{models, views, Context};

pub async fn workspace(
    _context: Context,
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> Result<impl warp::Reply, warp::Rejection> {
    let workspace_html = views::workspace::workspace_page(expanded_user, workspace);

    Ok(warp::reply::html(workspace_html))
}