use crate::{
    models,
    schema::{user, workspace},
    DOMAIN, USER_AGENT,
};
use diesel::prelude::*;
use rss::Image;

pub struct Feed {
    user: models::user::User,
    root: models::workspace::Workspace,
    items: Vec<models::workspace::Workspace>,
}

impl Feed {
    pub fn get_for_user(
        conn: &mut PgConnection,
        username: String,
    ) -> Result<Option<Self>, diesel::result::Error> {
        let (item, root) = diesel::alias!(workspace as item, workspace as root);
        user::table
            .left_join(
                root.on(root
                    .field(workspace::user_id)
                    .eq(user::id)
                    .and(root.field(workspace::type_id).eq(1))
                    .and(root.field(workspace::parent_id).eq(-1))),
            )
            .left_join(
                item.on(user::id
                    .eq(item.field(workspace::user_id))
                    .and(item.field(workspace::type_id).ne(1))
                    .and(item.field(workspace::is_published).eq(true))),
            )
            .filter(item.field(workspace::deleted_at).is_null())
            .filter(user::deleted_at.is_null())
            .filter(user::username.eq(username))
            .load::<(
                models::user::User,
                Option<models::workspace::Workspace>,
                Option<models::workspace::Workspace>,
            )>(conn)
            .map(|res| Self::from_joined(res))
    }

    fn from_joined(
        res: Vec<(
            models::user::User,
            Option<models::workspace::Workspace>,
            Option<models::workspace::Workspace>,
        )>,
    ) -> Option<Self> {
        if res.len() <= 0 {
            return None;
        }

        let r = res.clone();
        let (user, root) = match &r[0] {
            (user, Some(root), _) => (user, root),
            _ => return None,
        };

        let items: Vec<models::workspace::Workspace> = res
            .into_iter()
            .filter_map(|(_, _, workspace)| workspace)
            .collect();

        Some(Self {
            user: user.to_owned(),
            root: root.to_owned(),
            items,
        })
    }

    pub fn to_rss_channel(&self) -> rss::Channel {
        rss::ChannelBuilder::default()
            .title(self.root.name.clone())
            .description(self.root.description.clone())
            .generator(Some(USER_AGENT.to_string()))
            .link(format!("{}/{}", DOMAIN, self.user.username.clone()))
            .image(Some(rss::Image {
                url: format!("{}/digitheque.png", DOMAIN),
                title: String::from("A Digitheque Publicaiton"),
                link: format!("{}/digitheque.png", DOMAIN),
                description: Some(String::from("A Digitheque Publicaiton")),
                width: Some("1151px".to_string()),
                height: Some("625px".to_string()),
            }))
            .items(
                self.items
                    .iter()
                    .map(|workspace| workspace.to_rss_item(self.user.username.clone()))
                    .collect::<Vec<rss::Item>>(),
            )
            .build()
    }
}

#[derive(Clone)]
pub struct FeedWorkspace {
    pub user: models::user::User,
    pub workspace: models::workspace::Workspace,
}

impl FeedWorkspace {
    pub fn get_for_user(
        conn: &mut PgConnection,
        username: String,
        workspace_id: i32,
    ) -> Result<Option<Self>, diesel::result::Error> {
        workspace::table
            .left_join(user::table.on(user::id.eq(workspace::user_id)))
            .filter(workspace::deleted_at.is_null())
            .filter(user::deleted_at.is_null())
            // .filter(workspace::is_published.eq(true))
            .filter(workspace::id.eq(workspace_id))
            .filter(user::username.eq(username))
            .load::<(models::workspace::Workspace, Option<models::user::User>)>(conn)
            .map(|res| Self::from_joined(res))
    }

    fn from_joined(
        res: Vec<(models::workspace::Workspace, Option<models::user::User>)>,
    ) -> Option<Self> {
        if res.len() <= 0 {
            return None;
        }

        tracing::info!("{:?}", res);

        match &res[0] {
            (workspace, Some(user)) => Some(Self {
                workspace: workspace.to_owned(),
                user: user.to_owned(),
            }),
            _ => panic!("Damn something went wrong!"),
        }
    }
}

#[test]
fn test_diesel() {
    let sql = workspace::table
        .left_join(user::table.on(user::id.eq(workspace::user_id)))
        .filter(workspace::deleted_at.is_null())
        .filter(user::deleted_at.is_null())
        // .filter(workspace::is_published.eq(true))
        .filter(workspace::id.eq(3))
        .filter(user::username.eq("hg"));
    assert_eq!(
        diesel::debug_query::<diesel::pg::Pg, _>(&sql).to_string(),
        "SELECT `users`.`id`, `users`.`name` FROM `users` \
        WHERE (`users`.`id` = ?) -- binds: [1]"
    );
}
