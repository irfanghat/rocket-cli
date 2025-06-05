pub const CORS: &str = r#"#![allow(non_snake_case)]
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
"#;

pub const AUTH_GUARD: &str = r#"use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::utils::validate_token;

pub struct AuthenticaionClaims {
    pub credentials: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticaionClaims {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = req.cookies().get("auth_token");
        
        match cookie {
            Some(c) => match validate_token(c.value()).await {
                Ok(credentials) => Outcome::Success(AuthenticaionClaims { credentials }),
                Err(err_msg) => {
                    Outcome::Error((Status::Unauthorized, AuthError::InvalidToken(err_msg)))
                }
            },
            None => Outcome::Error((Status::Unauthorized, AuthError::MissingToken)),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken(String),
}"#;

pub const GITIGNORE: &str = r#"/target
.env
"#;