use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use lettre::{message::header::ContentType, AsyncTransport, Message};
use serde::Deserialize;
use tide::{prelude::json, Request, Response, StatusCode};

use crate::{
    db::{resets, users, Reset},
    State,
};

const RESET_PAGE: &str = include_str!("../templates/reset.hbs");

#[derive(Deserialize)]
struct ResetQuery {
    email: String,
}

pub async fn reset_password(req: Request<State>) -> tide::Result {
    let mut conn = req.state().pool.get().await?;
    let query: ResetQuery = req.query()?;
    let reg = handlebars::Handlebars::new();
    let user_id = match users::table
        .filter(users::email.eq(query.email.clone()))
        .select(users::id)
        .first::<String>(&mut conn)
        .await
        .optional()?
    {
        Some(user_id) => user_id,
        None => return Ok(Response::new(StatusCode::NotFound)),
    };
    let reset = Reset::new(user_id);
    let reset_url = format!(
        "{}/reset/{}?secret={}",
        req.state().config.domain,
        reset.id,
        reset.secret
    );
    diesel::insert_into(resets::table)
        .values(reset)
        .execute(&mut conn)
        .await?;
    let mail = Message::builder()
        .from(req.state().config.email.parse()?)
        .to(query.email.parse()?)
        .subject("Testing")
        .header(ContentType::TEXT_HTML)
        .body(reg.render_template(
            RESET_PAGE,
            &json!({
                "url" : reset_url,
            }),
        )?)?;
    req.state().smtp_transport.send(mail).await?;
    Ok(Response::new(StatusCode::Ok))
}

#[derive(Deserialize)]
struct ResetVerifyQuery {
    secret: String,
}

#[derive(Deserialize)]
struct ResetForm {
    password: String,
}
pub async fn reset(mut req: Request<State>) -> tide::Result {
    let reset_form: ResetForm = req.body_json().await?;
    let mut conn = req.state().pool.get().await?;
    let reset_id = req.param("reset_id")?;
    let secret = req.query::<ResetVerifyQuery>()?.secret;
    match resets::table
        .filter(resets::id.eq(reset_id))
        .first::<Reset>(&mut conn)
        .await
        .optional()?
    {
        Some(reset) if reset.secret == secret => {
            diesel::update(users::table)
                .set(users::password.eq(hash(reset_form.password, DEFAULT_COST)?))
                .execute(&mut conn)
                .await?;
            diesel::delete(resets::table)
                .filter(resets::id.eq(reset_id))
                .execute(&mut conn);
            return Ok(Response::new(StatusCode::Ok));
        }
        Some(_) => return Ok(Response::new(StatusCode::Unauthorized)),
        None => return Ok(Response::new(StatusCode::NotFound)),
    };
}
