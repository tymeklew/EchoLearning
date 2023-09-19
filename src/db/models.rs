use crate::utils;

use super::{resets, sessions, users};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Queryable, Selectable, PartialEq, Debug, Identifiable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub created_at: OffsetDateTime,
}
impl User {
    pub fn new(email: String, name: String, password: String) -> tide::Result<Self> {
        Ok(Self {
            email,
            name,
            password: hash(password, DEFAULT_COST)?,
            id: Uuid::new_v4().to_string(),
            created_at: OffsetDateTime::now_utc(),
        })
    }
}

#[derive(Queryable, Selectable, Associations, Debug, PartialEq, Identifiable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
}
impl Session {
    pub fn new(user_id: String) -> tide::Result<Self> {
        Ok(Self {
            user_id,
            id: Uuid::new_v4().to_string(),
            created_at: OffsetDateTime::now_utc(),
            expires_at: OffsetDateTime::now_utc() + time::Duration::days(30),
        })
    }
}

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq)]
#[diesel(table_name = resets)]
pub struct Reset {
    pub id: String,
    pub user_id: String,
    pub secret: String,
}
impl Reset {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            id: Uuid::new_v4().to_string(),
            secret: utils::generate_secret(),
        }
    }
}
