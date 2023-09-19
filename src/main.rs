mod account;
mod auth;
mod config;
mod db;
mod middleware;
mod questions;
mod utils;

use config::Config;
use diesel_async::pooled_connection::mobc::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenv::dotenv;
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use std::env::var;
use std::path::PathBuf;
use tide::{
    http::headers::HeaderValue,
    log::{self, info},
    security::{CorsMiddleware, Origin},
    Redirect, Server,
};

#[derive(Clone)]
pub struct State {
    pool: Pool<diesel_async::AsyncMysqlConnection>,
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    config: Config,
}
impl State {
    pub async fn connect() -> tide::Result<Self> {
        let config = Config::load();
        Ok(Self {
            pool: Pool::builder().build(AsyncDieselConnectionManager::<
                diesel_async::AsyncMysqlConnection,
            >::new(
                var("DATABASE_URL").unwrap_or("".to_string())
            )),
            smtp_transport: AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&match var(
                "EMAIL_RELAY",
            ) {
                Ok(relay) => relay,
                Err(_) => panic!("Please supply email relay url"),
            })?
            .credentials(Credentials::new(
                config.email.clone(),
                match var("EMAIL_PASSWORD") {
                    Ok(email_login) => email_login,
                    Err(e) => {
                        panic!(
                            "Please enter your email password into the .env file : {}",
                            e
                        );
                    }
                },
            ))
            .build(),
            config,
        })
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    log::start();

    let mut app = tide::with_state(State::connect().await?);
    let port = app.state().config.port.clone();

    //testing smtp connection to see if details work
    /*match app.state().smtp_transport.test_connection().await? {
        true => info!("SMTP connection working"),
        false => panic!("Smtp connection failed"),
    };*/

    //setting up CORS
    app.with(
        CorsMiddleware::new()
            .allow_credentials(true)
            .allow_origin(Origin::Any)
            .allow_headers("GET,POST".parse::<HeaderValue>()?),
    );

    app.with(middleware::user_loader);

    app.at("/account/me").get(account::me);
    app.at("/account/delete").post(account::delete);
    app.at("/verify").post(utils::verify);
    app.at("/reset").get(utils::reset_password);
    app.at("/reset/password/:reset_id").post(utils::reset);
    app.at("/auth/register").post(auth::register);
    app.at("/auth/login").post(auth::login);
    app.at("/auth/signout").post(auth::sign_out);

    //Serving client build
    serve_dir(
        &mut app,
        PathBuf::from("client/dist"),
        PathBuf::from("client/dist"),
    )?;
    app.at("*").serve_file("client/dist/index.html")?;
    app.at("/").get(Redirect::permanent("/index.html"));
    app.listen(port).await?;
    Ok(())
}

fn serve_dir(app: &mut Server<State>, path: PathBuf, parent: PathBuf) -> tide::Result<()> {
    for entry in path.read_dir()? {
        let e = entry?;
        let temp = e.path();
        let path = temp.strip_prefix(parent.clone())?.to_string_lossy();
        app.at(&format!("/{}", path)).serve_file(e.path())?;
        if e.metadata()?.is_dir() {
            serve_dir(app, e.path(), parent.clone())?;
        }
    }
    Ok(())
}
