use crate::{
    models,
    schema::{session, user},
    utils::{encrypt, now, sanitize_html, verify},
};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct NewUserApi {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

impl Into<UserCredentialsEncrypted> for NewUserApi {
    fn into(self) -> UserCredentialsEncrypted {
        UserCredentialsEncrypted {
            username: sanitize_html(&self.username),
            password: encrypt(&self.password),
        }
    }
}

#[derive(Deserialize)]
pub struct UserCredentialsApi {
    pub username: String,
    pub password: String,
}

impl Into<UserCredentialsEncrypted> for UserCredentialsApi {
    fn into(self) -> UserCredentialsEncrypted {
        UserCredentialsEncrypted {
            username: self.username,
            password: encrypt(&self.password),
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
            prelude: None,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<User, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn create(conn: &mut PgConnection, new_user: &NewUser) -> Result<User, diesel::result::Error> {
    diesel::insert_into(user::table)
        .values(new_user)
        .get_result(conn)
}

pub fn read(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    user::table.load::<User>(conn)
}

pub fn read_by_id(conn: &mut PgConnection, id: i32) -> Result<User, diesel::result::Error> {
    user::table
        .filter(user::id.eq(id))
        .filter(user::deleted_at.is_null())
        .first::<User>(conn)
}

pub fn read_by_credentials(
    conn: &mut PgConnection,
    credentials: UserCredentialsApi,
) -> Result<User, diesel::result::Error> {
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

pub fn delete(conn: &mut PgConnection, user: &User) -> QueryResult<usize> {
    diesel::update(user)
        .set((user::deleted_at.eq(Some(now())),))
        .execute(conn)
}

pub fn update(conn: &mut PgConnection, user: &mut User) -> QueryResult<usize> {
    diesel::update(user::table)
        .filter(user::id.eq(user.id))
        .set((user::updated_at.eq(Some(now())), user::style.eq(user.style.clone()),user::prelude.eq(user.prelude.clone())))
        .execute(conn)
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