pub const CARGO_TOML: &str = r#"[package]
name = "{{project_name}}"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = { version = "0.5.1", features = ["json"] }
"#;

pub const MAIN_RS: &str = r#"#[macro_use] 
extern crate rocket;

mod catchers;
mod db;
mod fairings;
mod guards;
mod models;
mod options;
mod repositories;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::init())
        .attach(fairings::cors)
         .register(
            "/",
            catchers![
                bad_request,
                unauthorized,
                forbidden,
                not_found,
                method_not_allowed,
                request_timeout,
                conflict,
                payload_too_large,
                unsupported_media_type,
                teapot,
                too_many_requests,
                internal_error,
                bad_gateway,
                service_unavailable,
                gateway_timeout
            ],
        )
        .mount("/", routes::routes())
}
"#;

pub const CATACHERS: &str = r#"use rocket::catch;

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
use mongodb::bson::oid::ObjectId;
use rocket::response::Responder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDocument {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(
        with = "bson::serde_helpers::chrono_datetime_as_bson_datetime",
        rename = "createdAt"
    )]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct UserInfo {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "createdAt")]
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

pub const ROUTES_MOD: &str = r#"use crate::guards::AuthClaims;
use crate::models::{ErrorResponse, SuccessResponse, UserInfo};
use crate::models::{LoginCredentials, RegistrationCredentials, User, UserDocument};
use crate::repositories::users::UserRepository;
use crate::utils::{authorize_user, hash_password};

use rocket::http::Status;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Responder;
use rocket::response::status::Custom;
use rocket::serde::{Deserialize, json::Json};
use rocket::{State, delete, get, post, put, routes};

use std::sync::Arc;

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

    let user = match repo
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
    let user_document = match repo.get_user_by_email(&credentials.email).await {
        Ok(Some(user_document)) => user_document,
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
        id: user_document.id.to_string(),
        username: user_document.username.to_string(),
        email: user_document.email.clone(),
        password: user_document.password.clone(),
        created_at: user_document.created_at.to_rfc3339(),
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
    let mut cookie = Cookie::build(("auth_token", token.clone()))
        .http_only(true)
        .secure(false) // Ensure your app uses HTTPS
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
    cookies.remove(Cookie::build(("auth_token", "")).path("/").finish());
    Json(SuccessResponse {
        status: 200,
        message: "Logged out successfully".to_string(),
    })
}

/// Retrieves a list of all users.
#[get("/users")]
pub async fn list_users(
    repo: &State<Arc<UserRepository>>,
) -> Result<Json<Vec<UserDocument>>, Json<ErrorResponse>> {
    let users = match repo.list_users().await {
        Ok(users) => users,
        Err(_) => {
            return Err(Json(ErrorResponse {
                status: Status::InternalServerError.code,
                message: "Something went wrong, please try again later".to_string(),
            }));
        }
    };

    Ok(Json(users))
}

/// Retrieves a single user by ID (requires authentication).
#[get("/users/<id>")]
pub async fn get_user(
    auth: AuthClaims,
    repo: &State<Arc<UserRepository>>,
    id: &str,
) -> Result<Json<UserDocument>, Json<ErrorResponse>> {
    let user = match repo.get_user_by_id(&id).await {
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
    auth: AuthClaims,
    repo: &State<Arc<UserRepository>>,
    email: &str,
) -> Result<Json<UserInfo>, Json<ErrorResponse>> {
    let user = match repo.get_user_by_email(&email).await {
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
        created_at: user.created_at.to_string(),
    }))
}

/// Updates an existing user's information by ID (requires authentication).
#[put("/update/<id>", data = "<credentials>")]
pub async fn update_user(
    auth: AuthClaims,
    repo: &State<Arc<UserRepository>>,
    id: &str,
    credentials: Json<RegistrationCredentials>,
) -> Result<Json<UserDocument>, Json<ErrorResponse>> {
    // Check if the email is already in use by another user
    if let Ok(Some(existing_user)) = repo.get_user_by_email(&credentials.email).await {
        // If the email exists and it's not the user being updated
        if existing_user.id.to_string() != id {
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
            &id,
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
    auth: AuthClaims,
    repo: &State<Arc<UserRepository>>,
    id: &str,
) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
    match repo.delete_user(&id).await {
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
use mongodb::{Client, Database, options::ClientOptions};
use rocket::fairing::AdHoc;
use std::{env, sync::Arc};

use crate::repositories::users::UserRepository;

pub fn init() -> AdHoc {
    AdHoc::on_ignite(
        "Establish connection with Database cluster",
        |rocket| async {
            match connect().await {
                Ok(user_repository) => rocket.manage(user_repository),
                Err(error) => {
                    panic!("Cannot connect to instance -> {:?}", error)
                }
            }
        },
    )
}

async fn connect() -> mongodb::error::Result<Arc<UserRepository>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set...");
    let database_name =
        std::env::var("DATABASE").expect("DATABASE must be set...");
    let client_options = ClientOptions::parse(database_url).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(&database_name);

    Ok(Arc::new(UserRepository::new(
        &client,
        &database_name,
        "users",
    )))
}
"#;

pub const REPOSITORIES: &str = r#"use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::{
    Client, Collection,
    bson::{doc, oid::ObjectId},
    error::{Error, Result},
    options::ClientOptions,
};
use serde::{Deserialize, Serialize};

use crate::models::UserDocument;

#[derive(Debug)]
pub struct UserRepository {
    collection: Collection<UserDocument>,
}

impl UserRepository {
    pub fn new(client: &Client, db_name: &str, collection_name: &str) -> Self {
        let collection = client
            .database(db_name)
            .collection::<UserDocument>(collection_name);
        Self { collection }
    }

    /// CREATE a new user
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<UserDocument> {
        if let Some(_) = self.collection.find_one(doc! { "email": email }).await? {
            return Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "A user with this email already exists.",
            )));
        }

        let user = UserDocument {
            id: ObjectId::new(),
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            created_at: Utc::now(),
        };

        self.collection.insert_one(&user).await?;

        Ok(user)
    }

    /// GET user by id
    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<UserDocument>> {
        match ObjectId::parse_str(id) {
            Ok(object_id) => {
                let filter = doc! { "_id": object_id };
                let user = self.collection.find_one(filter).await?;
                Ok(user)
            }
            Err(_) => Ok(None), // Invalid ID treated as "not found"
        }
    }

    /// GET user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserDocument>> {
        let filter = doc! { "email": email };
        let user = self.collection.find_one(filter).await?;
        Ok(user)
    }

    /// UPDATE a user
    pub async fn update_user(
        &self,
        id: &str,
        username: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<Option<UserDocument>> {
        let object_id = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return Ok(None),
        };

        let mut update_doc = doc! {};

        if let Some(username) = username {
            update_doc.insert("username", username);
        }

        if let Some(email) = email {
            update_doc.insert("email", email);
        }

        if let Some(password) = password {
            update_doc.insert("password", password);
        }

        if update_doc.is_empty() {
            return Ok(None);
        }

        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": update_doc };

        let user = self.collection.find_one_and_update(filter, update).await?;
        Ok(user)
    }

    /// DELETE a user
    pub async fn delete_user(&self, id: &str) -> Result<Option<UserDocument>> {
        let object_id = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return Ok(None),
        };

        let filter = doc! { "_id": object_id };
        let user = self.collection.find_one_and_delete(filter).await?;
        Ok(user)
    }

    /// GET all users
    pub async fn list_users(&self) -> Result<Vec<UserDocument>> {
        let filter = doc! {};
        let mut cursor = self.collection.find(filter).await?;
        let mut users = Vec::new();

        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }

        Ok(users)
    }
}
"#;
