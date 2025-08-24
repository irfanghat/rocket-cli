pub const CARGO_TOML: &str = r#"[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"

[dependencies]
argon2 = "0.5.3"
bcrypt = "0.16.0"
chrono = { version = "0.4.39", features = ["serde"] }
dotenvy = "0.15.7"
futures = "0.3.31"
jsonwebtoken = "9.3.0"
rand = "0.8.5"
regex = "1.11.1"
rocket = { version = "0.5.1", features = ["json"] }
schemars = "0.8.21"
serde = { version = "1.0.216", features = ["derive"] }
tokio = { version = "1.42.0", features = ["full"] }
sha2 = "0.10.8"
uuid = { version = "1.11.0", features = ["v4", "serde"] }
rbatis = "4.6"
rbdc-pg = "4.6"
rbs = "4.6"
"#;

pub const MAIN_RS: &str = r#"#[macro_use] 
extern crate rocket;

mod auth;
mod catchers;
mod db;
mod fairings;
mod guards;
mod middleware;
mod models;
mod options;
mod repositories;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::init())
        .attach(fairings::Cors)
        .register(
            "/",
            catchers![
                catchers::bad_request,
                catchers::unauthorized,
                catchers::forbidden,
                catchers::not_found,
                catchers::method_not_allowed,
                catchers::request_timeout,
                catchers::conflict,
                catchers::payload_too_large,
                catchers::unsupported_media_type,
                catchers::teapot,
                catchers::too_many_requests,
                catchers::internal_error,
                catchers::bad_gateway,
                catchers::service_unavailable,
                catchers::gateway_timeout
            ],
        )
        .mount("/", routes![options::options])
        .mount("/", routes::user_routes())
}
"#;

pub const CATCHERS: &str = r#"use rocket::catch;

#[catch(400)]
pub async fn bad_request() -> &'static str {
    "Bad Request."
}

#[catch(401)]
pub async fn unauthorized() -> &'static str {
    "Unauthorized access."
}

#[catch(403)]
pub async fn forbidden() -> &'static str {
    "You don't have permission to access this resource."
}

#[catch(404)]
pub async fn not_found() -> &'static str {
    "Resource not found."
}

#[catch(405)]
pub async fn method_not_allowed() -> &'static str {
    "Method Not Allowed."
}

#[catch(408)]
pub async fn request_timeout() -> &'static str {
    "Request Timeout."
}

#[catch(409)]
pub async fn conflict() -> &'static str {
    "The request could not be completed due to a conflict."
}

#[catch(413)]
pub async fn payload_too_large() -> &'static str {
    "Payload Too Large."
}

#[catch(415)]
pub async fn unsupported_media_type() -> &'static str {
    "Unsupported Media Type."
}

#[catch(418)]
pub async fn teapot() -> &'static str {
    "I'm a teapot."
}

#[catch(429)]
pub async fn too_many_requests() -> &'static str {
    "Too Many Requests."
}

#[catch(500)]
pub async fn internal_error() -> &'static str {
    "Internal Server Error."
}

#[catch(502)]
pub async fn bad_gateway() -> &'static str {
    "Bad Gateway."
}

#[catch(503)]
pub async fn service_unavailable() -> &'static str {
    "Service Unavailable."
}

#[catch(504)]
pub async fn gateway_timeout() -> &'static str {
    "Gateway Timeout."
}
"#;

pub const MODELS: &str = r#"use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database entity struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

/// DTO with password included
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
}

/// DTO without password
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct RegistrationCredentials {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessResponse {
    pub status: u16,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}
"#;

pub const OPTIONS: &str = r#"
#[rocket::options("/<_route_args..>")]
pub async fn options(_route_args: Option<std::path::PathBuf>) -> rocket::http::Status {
    rocket::http::Status::Ok
}
"#;

pub const ROUTES_MOD: &str = r#"use crate::auth::{authorize_user, hash_password};
use crate::guards::AuthClaims;
use crate::models::{ErrorResponse, SuccessResponse, UserInfo};
use crate::models::{LoginCredentials, RegistrationCredentials, User, UserEntity};
use crate::repositories::UserRepository;

use rocket::http::Status;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::serde::json::Json;
use rocket::{State, delete, get, post, put, routes};

use std::sync::Arc;
use uuid::Uuid;

/// Registers a new user.
#[post("/register", data = "<credentials>")]
pub async fn register(
    repo: &State<Arc<UserRepository>>,
    credentials: Json<RegistrationCredentials>,
) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
    if let Ok(Some(_)) = repo.get_user_by_email(&credentials.email).await {
        return Err(Json(ErrorResponse {
            status: Status::Conflict.code,
            message: "A user with this email already exists".to_string(),
        }));
    }

    let hashed_password = match hash_password(credentials.password.clone()) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Something went wrong, please try again later".to_string(),
            }));
        }
    };

    let _ = match repo
        .create_user(&credentials.username, &credentials.email, &hashed_password)
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Failed to register account".to_string(),
            }));
        }
    };

    Ok(Json(SuccessResponse {
        status: Status::Ok.code,
        message: "User registered successfully".to_string(),
    }))
}

/// Authenticates a user and sets an authentication cookie.
#[post("/login", data = "<credentials>")]
pub async fn login(
    repo: &State<Arc<UserRepository>>,
    credentials: Json<LoginCredentials>,
    cookies: &CookieJar<'_>,
) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
    let user_entity = match repo.get_user_by_email(&credentials.email).await {
        Ok(Some(user_entity)) => user_entity,
        Ok(None) => {
            return Err(Json(ErrorResponse {
                status: Status::Unauthorized.code,
                message: "Invalid email or password".to_string(),
            }));
        }
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Something went wrong, please try again later".to_string(),
            }));
        }
    };

    let user = User {
        id: user_entity.id.to_string(),
        username: user_entity.username.clone(),
        email: user_entity.email.clone(),
        password: user_entity.password.clone(),
        created_at: user_entity.created_at.to_rfc3339(),
    };

    let token = match authorize_user(&user, &credentials).await {
        Ok(token) => token,
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::Unauthorized.code,
                message: "Invalid email or password".to_string(),
            }));
        }
    };

    // Set the token cookie (HTTP-only, secure)
    #[allow(deprecated)]
    let cookie = Cookie::build(("auth_token", token.clone()))
        .http_only(true)
        .secure(false) // Set to true in production with HTTPS
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    cookies.add(cookie);

    Ok(Json(SuccessResponse {
        status: Status::Ok.code,
        message: "Login successful".to_string(),
    }))
}

/// Logs out the current user by removing the authentication cookie.
#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Json<SuccessResponse> {
    cookies.remove(Cookie::build(("auth_token", "")).path("/").build());
    Json(SuccessResponse {
        status: 200,
        message: "Logged out successfully".to_string(),
    })
}

/// Retrieves a single user by ID (requires authentication).
#[get("/users/<id>")]
pub async fn get_user(
    _auth: AuthClaims,
    repo: &State<Arc<UserRepository>>,
    id: &str,
) -> Result<Json<UserEntity>, Json<ErrorResponse>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::BadRequest.code,
                message: "Invalid user ID format".to_string(),
            }));
        }
    };

    let user = match repo.get_user_by_id(uuid).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(Json(ErrorResponse {
                status: Status::NotFound.code,
                message: "User not found".to_string(),
            }));
        }
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Something went wrong, please try again later".to_string(),
            }));
        }
    };

    Ok(Json(user))
}

/// Retrieves a single user by email (requires authentication).
#[get("/user/<email>")]
pub async fn get_user_by_email(
    _auth: AuthClaims,
    repo: &State<Arc<UserRepository>>,
    email: &str,
) -> Result<Json<UserInfo>, Json<ErrorResponse>> {
    let user = match repo.get_user_by_email(email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(Json(ErrorResponse {
                status: Status::NotFound.code,
                message: "User not found".to_string(),
            }));
        }
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Something went wrong, please try again later".to_string(),
            }));
        }
    };

    Ok(Json(UserInfo {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        created_at: user.created_at.to_rfc3339(),
    }))
}

/// Updates an existing user's information by ID (requires authentication).
#[put("/update/<id>", data = "<credentials>")]
pub async fn update_user(
    _auth: AuthClaims,
    repo: &State<Arc<UserRepository>>,
    id: &str,
    credentials: Json<RegistrationCredentials>,
) -> Result<Json<UserEntity>, Json<ErrorResponse>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::BadRequest.code,
                message: "Invalid user ID format".to_string(),
            }));
        }
    };

    // Check if the email is already in use by another user
    if let Ok(Some(existing_user)) = repo.get_user_by_email(&credentials.email).await {
        // If the email exists and it's not the user being updated
        if existing_user.id != uuid {
            return Err(Json(ErrorResponse {
                status: Status::Conflict.code,
                message: "A user with this email already exists".to_string(),
            }));
        }
    }

    let hashed_password = match hash_password(credentials.password.clone()) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Something went wrong, please try again later".to_string(),
            }));
        }
    };

    let user = match repo
        .update_user(
            uuid,
            Some(&credentials.username),
            Some(&credentials.email),
            Some(&hashed_password),
        )
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(Json(ErrorResponse {
                status: Status::NotFound.code,
                message: "User not found".to_string(),
            }));
        }
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Something went wrong, please try again later".to_string(),
            }));
        }
    };

    Ok(Json(user))
}

/// Deletes a user by ID (requires authentication).
#[delete("/delete/<id>")]
pub async fn delete_user(
    _auth: AuthClaims,
    repo: &State<Arc<UserRepository>>,
    id: &str,
) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
    let uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::BadRequest.code,
                message: "Invalid user ID format".to_string(),
            }));
        }
    };

    match repo.delete_user(uuid).await {
        Ok(Some(_)) => Ok(Json(SuccessResponse {
            status: Status::Ok.code,
            message: "User deleted successfully".to_string(),
        })),
        Ok(None) => {
            return Err(Json(ErrorResponse {
                status: Status::NotFound.code,
                message: "User not found".to_string(),
            }));
        }
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Something went wrong, please try again later".to_string(),
            }));
        }
    }
}

/// Collects all user-related routes for mounting.
pub fn user_routes() -> Vec<rocket::Route> {
    routes![
        register,
        login,
        logout,
        get_user,
        get_user_by_email,
        update_user,
        delete_user
    ]
}
"#;

pub const DB: &str = r#"use dotenvy::dotenv;
use rbatis::RBatis;
use rbdc_pg::driver::PgDriver;
use rocket::fairing::AdHoc;
use std::sync::Arc;

use crate::repositories::UserRepository;

pub fn init() -> AdHoc {
    AdHoc::on_ignite(
        "Establish connection with PostgreSQL database",
        |rocket| async {
            match connect().await {
                Ok(user_repository) => rocket.manage(user_repository),
                Err(error) => {
                    panic!("Cannot connect to database -> {:?}", error)
                }
            }
        },
    )
}

async fn connect() -> Result<Arc<UserRepository>, rbatis::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set...");

    let rb = RBatis::new();
    rb.link(PgDriver {}, &database_url).await?;

    Ok(Arc::new(UserRepository::new(rb)))
}
"#;

pub const REPOSITORIES: &str = r#"use crate::models::UserEntity;
use chrono::Utc;
use rbatis::{raw_sql, RBatis};
use uuid::Uuid;

pub struct UserRepository {
    rb: RBatis,
}

impl UserRepository {
    pub fn new(rb: RBatis) -> Self {
        Self { rb }
    }

    //----------------------------------
    // Create a new user
    //----------------------------------
    raw_sql!(insert_user(rb: &RBatis, user: &UserEntity) -> rbatis::rbdc::db::ExecResult =>
        "INSERT INTO users (id, username, email, password, created_at) VALUES (?, ?, ?, ?, ?)"
    );

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<UserEntity, rbatis::Error> {
        //-------------------------------------------
        // Check for existing user
        //-------------------------------------------
        if let Some(_) = self.get_user_by_email(email).await? {
            return Err(rbatis::Error::from("A user with this email already exists"));
        }

        let user = UserEntity {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            created_at: Utc::now(),
        };

        Self::insert_user(&self.rb, &user).await?;
        Ok(user)
    }

    //----------------------------------------------
    // Get user by id
    //----------------------------------------------
    raw_sql!(get_by_id(rb: &RBatis, id: Uuid) -> Option<UserEntity> =>
        "SELECT id, username, email, password, created_at FROM users WHERE id = ?"
    );

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserEntity>, rbatis::Error> {
        Self::get_by_id(&self.rb, id).await
    }

    //-------------------------------------------------
    // Get user by email
    //-------------------------------------------------
    raw_sql!(get_by_email(rb: &RBatis, email: &str) -> Option<UserEntity> =>
        "SELECT id, username, email, password, created_at FROM users WHERE email = ?"
    );

    pub async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<UserEntity>, rbatis::Error> {
        Self::get_by_email(&self.rb, email).await
    }

    //----------------------------------
    // Update user
    //----------------------------------
    raw_sql!(update_user_sql(
        rb: &RBatis,
        id: Uuid,
        username: &str,
        email: &str,
        password: &str
    ) -> rbatis::rbdc::db::ExecResult =>
        "UPDATE users SET username = ?, email = ?, password = ? WHERE id = ?"
    );

    pub async fn update_user(
        &self,
        id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<Option<UserEntity>, rbatis::Error> {
        let mut user = match self.get_user_by_id(id).await? {
            Some(u) => u,
            None => return Ok(None),
        };

        if let Some(u) = username {
            user.username = u.to_string();
        }
        if let Some(e) = email {
            user.email = e.to_string();
        }
        if let Some(p) = password {
            user.password = p.to_string();
        }

        Self::update_user_sql(
            &self.rb,
            user.id,
            &user.username,
            &user.email,
            &user.password,
        )
        .await?;

        Ok(Some(user))
    }

    //-------------------------
    // Delete user
    //-------------------------
    raw_sql!(delete_user_sql(rb: &RBatis, id: Uuid) -> rbatis::rbdc::db::ExecResult =>
        "DELETE FROM users WHERE id = ?"
    );

    pub async fn delete_user(&self, id: Uuid) -> Result<Option<UserEntity>, rbatis::Error> {
        if let Some(user) = self.get_user_by_id(id).await? {
            Self::delete_user_sql(&self.rb, id).await?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    //--------------------------------------
    // List all users
    //--------------------------------------
    raw_sql!(list_users_sql(rb: &RBatis) -> Vec<UserEntity> =>
        "SELECT id, username, email, password, created_at FROM users"
    );

    pub async fn list_users(&self) -> Result<Vec<UserEntity>, rbatis::Error> {
        Self::list_users_sql(&self.rb).await
    }
}
"#;

pub const MIGRATIONS: &str = r#"-- Create users table migration
-- File: migrations/001_create_users_table.sql

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
"#;

pub const ENV_TEMPLATE: &str = r#"# Database Configuration
#--------------------------------------
# Database Configuration
#--------------------------------------
DATABASE_URL=postgresql://postgres:mysecretpassword@localhost/postgres

#--------------------------------------
# Generate a secure random string
# openssl rand 32 -base64
#--------------------------------------
JWT_SECRET=your-super-secret-jwt-key-here

#--------------------------------------
# App Configuration
#--------------------------------------
ROCKET_PORT=8000
ROCKET_ADDRESS=0.0.0.0
"#;