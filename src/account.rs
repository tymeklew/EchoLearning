use bcrypt::verify;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, StatusCode};

use crate::{
    db::{resets, sessions, users, User},
    State,
};

#[derive(Serialize)]
struct UserDetails {
    email: String,
    name: String,
}

pub async fn me(req: Request<State>) -> tide::Result {
    let user: &User = match req.ext() {
        Some(user) => user,
        None => return Ok(Response::new(StatusCode::Unauthorized)),
    };
    let details: UserDetails = UserDetails {
        email: user.email.clone(),
        name: user.name.clone(),
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(Body::from_json(&details)?)
        .build())
}

#[derive(Deserialize)]
struct DeleteForm {
    password: String,
}
pub async fn delete(mut req: Request<State>) -> tide::Result {
    let form: DeleteForm = req.body_json().await?;
    let mut conn = req.state().pool.get().await?;
    let user: &User = match req.ext() {
        Some(user) => user,
        None => return Ok(Response::new(StatusCode::Unauthorized)),
    };
    if !verify(form.password.clone(), &user.password)? {
        return Ok(Response::new(StatusCode::Unauthorized));
    }
    diesel::delete(sessions::table)
        .filter(sessions::user_id.eq(user.id.clone()))
        .execute(&mut conn)
        .await?;
    diesel::delete(resets::table)
        .filter(resets::user_id.eq(user.id.clone()))
        .execute(&mut conn)
        .await?;
    diesel::delete(users::table)
        .filter(users::id.eq(user.id.clone()))
        .execute(&mut conn)
        .await?;

    return Ok(Response::new(StatusCode::Ok));
}
