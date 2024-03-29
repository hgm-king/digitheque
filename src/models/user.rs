use crate::{
    models,
    schema::{session, user},
    utils::{encrypt, now, sanitize_html, verify},
    DEFAULT_PRELUDE_CONTENT,
};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use html_to_string_macro::html;

#[derive(Clone, Debug, Identifiable, Queryable, Selectable)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub style: Option<String>,
    pub prelude: Option<String>,
}

impl User {
    pub fn new(conn: &mut PgConnection, new_user: &NewUser) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(user::table)
            .values(new_user)
            .get_result(conn)
    }
    
    pub fn read_by_id(conn: &mut PgConnection, id: i32) -> Result<Self, diesel::result::Error> {
        user::table
            .filter(user::id.eq(id))
            .filter(user::deleted_at.is_null())
            .first::<Self>(conn)
    }
    
    pub fn read_by_username(conn: &mut PgConnection, username: String) -> Result<Self, diesel::result::Error> {
        user::table
            .filter(user::username.eq(username))
            .filter(user::deleted_at.is_null())
            .first::<Self>(conn)
    }
    
    pub fn read_by_credentials(
        conn: &mut PgConnection,
        credentials: UserCredentialsApi,
    ) -> Result<Self, diesel::result::Error> {
        let user: User = user::table
            .filter(user::username.eq(credentials.username))
            .filter(user::deleted_at.is_null())
            .first::<User>(conn)?;
    
        if verify(&credentials.password, &user.password) {
            Ok(user)
        } else {
            Err(diesel::NotFound)
        }
    }

    pub fn delete(&self, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::update(self)
            .set((user::deleted_at.eq(Some(now())),))
            .execute(conn)
    }
    
    pub fn update(&self, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::update(user::table)
            .filter(user::id.eq(self.id))
            .set((
                user::updated_at.eq(Some(now())),
                user::style.eq(self.style.clone()),
                user::prelude.eq(self.prelude.clone()),
            ))
            .execute(conn)
    }

    pub fn link_to_prelude() -> String {
        html! {
            <a href="/prelude">"Edit prelude"</a>
        }
    }

    pub fn link_to_stylesheet() -> String {
        html! {
            <a href="/stylesheet">"Edit stylesheet"</a>
        }
    }

    pub fn link_to_feed() -> String {
        html! {
            <a href="/feed">"Edit feed"</a>
        }
    }
}

#[derive(Deserialize)]
pub struct NewUserApi {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

impl From<NewUserApi> for UserCredentialsEncrypted {
    fn from(creds: NewUserApi) -> Self {
        UserCredentialsEncrypted {
            username: sanitize_html(&creds.username),
            password: encrypt(&creds.password),
        }
    }
}

#[derive(Deserialize)]
pub struct UserCredentialsApi {
    pub username: String,
    pub password: String,
}

impl From<UserCredentialsApi> for UserCredentialsEncrypted {
    fn from(creds: UserCredentialsApi) -> Self {
        UserCredentialsEncrypted {
            username: creds.username,
            password: encrypt(&creds.password),
        }
    }
}

pub struct UserCredentialsEncrypted {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct ExpandedUser {
    pub user: User,
    pub session: models::session::Session,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub style: Option<String>,
    pub prelude: Option<String>,
}

impl NewUser {
    pub fn new(new_user: UserCredentialsEncrypted) -> Self {
        NewUser {
            username: new_user.username,
            password: new_user.password,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            style: None,
            prelude: Some(DEFAULT_PRELUDE_CONTENT.to_string()),
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<User, diesel::result::Error> {
        User::new(conn, self)
    }
}

pub fn read_user_by_session(
    conn: &mut PgConnection,
    session_id: i32,
) -> Result<ExpandedUser, diesel::result::Error> {
    let r = user::table
        .inner_join(session::table.on(user::id.eq(session::user_id)))
        // .filter(session::valid_until.gt(now()))
        .filter(session::deleted_at.is_null())
        .filter(session::id.eq(session_id))
        .filter(user::deleted_at.is_null())
        .select((User::as_select(), models::session::Session::as_select()))
        .first(conn);

    r.map(|(user, session)| ExpandedUser { user, session })
}

pub fn cleanup_table(conn: &mut PgConnection) {
    diesel::delete(user::table).execute(conn).unwrap();
}

#[derive(Deserialize)]
pub struct UpdateStyleApi {
    pub style: String,
}

#[derive(Deserialize)]
pub struct UpdatePreludeApi {
    pub prelude: String,
}
