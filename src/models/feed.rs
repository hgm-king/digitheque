use crate::{
    models,
    schema::{user, workspace},
    DOMAIN, USER_AGENT,
};
use diesel::prelude::*;

pub struct Feed {
    user: models::user::User,
    root: models::workspace::Workspace,
    items: Vec<models::workspace::Workspace>,
}

impl Feed {
    pub fn get_for_user(
        conn: &mut PgConnection,
        username: String,
    ) -> Result<Self, diesel::result::Error> {
        let (item, root) = diesel::alias!(workspace as parent, workspace as children);
        item.left_join(user::table.on(user::id.eq(item.field(workspace::user_id))))
            .left_join(root.on(root.field(workspace::user_id).eq(user::id)))
            .filter(item.field(workspace::deleted_at).is_null())
            .filter(user::deleted_at.is_null())
            .filter(item.field(workspace::is_published).eq(true))
            .filter(item.field(workspace::type_id).ne(1))
            .filter(root.field(workspace::type_id).eq(1))
            .filter(root.field(workspace::parent_id).eq(-1))
            .filter(user::username.eq(username))
            .load::<(
                models::workspace::Workspace,
                Option<models::user::User>,
                Option<models::workspace::Workspace>,
            )>(conn)
            .map(|res| Self::from_joined(res))
    }

    fn from_joined(
        res: Vec<(
            models::workspace::Workspace,
            Option<models::user::User>,
            Option<models::workspace::Workspace>,
        )>,
    ) -> Self {
        if res.len() <= 0 {
            // none are published....
        }

        let r = res.clone();
        let (user, root) = match &r[0] {
            (_, Some(user), Some(root)) => (user, root),
            _ => panic!("Damn something went wrong!"),
        };

        let items: Vec<models::workspace::Workspace> =
            res.into_iter().map(|(workspace, _, _)| workspace).collect();

        Self {
            user: user.to_owned(),
            root: root.to_owned(),
            items,
        }
    }

    pub fn to_rss_channel(&self) -> rss::Channel {
        rss::ChannelBuilder::default()
            .title(self.root.name.clone())
            .description(self.root.description.clone())
            .generator(Some(USER_AGENT.to_string()))
            .link(format!("{}/{}", DOMAIN, self.user.username.clone()))
            .items(
                self.items
                    .iter()
                    .map(|workspace| workspace.to_rss_item(self.user.username.clone()))
                    .collect::<Vec<rss::Item>>(),
            )
            .build()
    }
}
