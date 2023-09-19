use crate::db::{sessions, Session};
use crate::db::{users, User};
use bcrypt::verify;
use diesel::prelude::*;
use diesel::result::OptionalExtension;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use tide::http::cookies::CookieBuilder;
use tide::Response;
use tide::StatusCode;
use tide::{Redirect, Request};

use crate::State;

#[derive(Deserialize)]
struct RegisterForm {
    email: String,
    password: String,
    name: String,
}

pub async fn register(mut req: Request<State>) -> tide::Result {
    let form: RegisterForm = req.body_json().await?;
    let mut conn = req.state().pool.get().await?;

    if users::table
        .filter(users::email.eq(&form.email))
        .count()
        .get_result::<i64>(&mut conn)
        .await?
        > 1
    {
        return Ok(Response::new(StatusCode::Conflict));
    };
    diesel::insert_into(users::table)
        .values(User::new(form.email, form.name, form.password)?)
        .execute(&mut conn)
        .await?;

    Ok(Response::new(StatusCode::Created))
}

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}
pub async fn login(mut req: Request<State>) -> tide::Result {
    let form: LoginForm = req.body_json().await?;
    let mut conn = req.state().pool.get().await?;

    match users::table
        .filter(users::email.eq(&form.email))
        .first::<User>(&mut conn)
        .await
        .optional()?
    {
        Some(user) if verify(form.password, &user.password)? => {
            let session: Session = Session::new(user.id)?;
            let session_id: String = session.id.clone();
            diesel::insert_into(sessions::table)
                .values(session)
                .execute(&mut conn)
                .await?;

            let mut res = Response::new(StatusCode::Ok);
            res.insert_cookie(
                CookieBuilder::new("session_id", session_id)
                    .path("/")
                    .finish(),
            );
            return Ok(res);
        }
        Some(_) => return Ok(Response::new(StatusCode::Unauthorized)),
        None => return Ok(Response::new(StatusCode::NotFound)),
    }
}
pub async fn sign_out(req: Request<State>) -> tide::Result {
    let session_id = match req.cookie("session_id") {
        Some(session_id) => session_id,
        None => return Ok(Response::new(StatusCode::Unauthorized)),
    }
    .value()
    .to_string();
    let mut conn = req.state().pool.get().await?;
    diesel::delete(sessions::table.filter(sessions::id.eq(session_id)))
        .execute(&mut conn)
        .await?;
    Ok(Redirect::new("/login").into())
}
