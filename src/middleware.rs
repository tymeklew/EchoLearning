use diesel::QueryDsl;
use std::future::Future;
use std::pin::Pin;
use tide::Result;

use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;
use tide::Next;
use tide::Request;

use crate::db::sessions;
use crate::db::{users, User};
use crate::State;

pub fn user_loader<'a>(
    mut request: Request<State>,
    next: Next<'a, State>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        let mut conn = request.state().pool.get().await?;
        let user: User = match request.cookie("session_id") {
            Some(session_id) => match sessions::table
                .inner_join(users::table)
                .filter(sessions::id.eq(session_id.value().to_string()))
                .select(User::as_select())
                .first::<User>(&mut conn)
                .await
                .optional()?
            {
                Some(user) => user,
                None => return Ok(next.run(request).await),
            },
            None => return Ok(next.run(request).await),
        };
        request.set_ext(user);
        Ok(next.run(request).await)
    })
}
