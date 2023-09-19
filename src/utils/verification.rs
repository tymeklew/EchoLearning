use crate::State;
use lettre::{message::header::ContentType, AsyncTransport, Message};
use tide::{prelude::json, Request};

const VERIFY_PAGE: &str = include_str!("../templates/verify.hbs");

pub async fn verify(req: Request<State>) -> tide::Result {
    let reg = handlebars::Handlebars::new();

    let mail = Message::builder()
        .from(req.state().config.email.parse()?)
        .to("tymek.lewandowski@gmail.com".parse()?)
        .subject("Testing")
        .header(ContentType::TEXT_HTML)
        .body(reg.render_template(
            VERIFY_PAGE,
            &json!({
                "email" : req.param("email").unwrap_or("tymek.lewandowski@gmail.com"),
            }),
        )?)?;
    req.state().smtp_transport.send(mail).await?;
    Ok("real".into())
}
