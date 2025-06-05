pub const CORS: &str = r#"use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add cors headers to responses",
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

use crate::middleware::validate_token;

pub struct AuthClaims {
    pub credentials: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthClaims {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = req.cookies().get("auth_token");
        
        match cookie {
            Some(c) => match validate_token(c.value()).await {
                Ok(credentials) => Outcome::Success(AuthClaims { credentials }),
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

pub const MIDDLEWARE: &str = r#"pub async fn validate_token(token: &str) -> Result<String, String> {
    let auth_key = std::env::var("AUTH_KEY")
        .map_err(|_| "AUTH_KEY must be set".to_string())?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["your-audience"]);

    let mut iss_set = HashSet::new();
    iss_set.insert("your-issuer".to_string());
    validation.iss = Some(iss_set);

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(auth_key.as_bytes()),
        &validation,
    )
    .map_err(|e| match e.kind() {
        ErrorKind::ExpiredSignature => "Token expired".to_string(),
        ErrorKind::InvalidToken => "Invalid token".to_string(),
        _ => format!("Token error: {}", e),
    })?;

    Ok(decoded.claims.sub)
}
"#;

pub const BASIC_AUTH: &str = r#"use crate::{
    guards::AuthClaims,
    models::{LoginCredentials, User},
};

use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use std::collections::{HashMap, HashSet};

/// JWT claims structure, including subject, expiration, and unique nonce.
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,        // Subject (user email)
    exp: usize,         // Expiration timestamp
    nonce: String,      // Unique secret marker
    aud: Vec<String>,   // Audience restriction
    iss: String,        // Issuer restriction
}

/// Authorizes a user by verifying credentials and generating a JWT.
pub async fn authorize_user(user: &User, credentials: &LoginCredentials) -> Result<String, String> {
    let auth_key = std::env::var_os("AUTH_KEY")
        .expect("[AUTH_KEY] must be set...")
        .into_string()
        .unwrap();

    // Verify the provided password against the stored hash.
    if !verify(&credentials.password, &user.password).map_err(|e| e.to_string())? {
        return Err("Invalid credentials".into());
    }

    // Generate a unique per-token nonce using user email and secret key.
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", user.email, auth_key));
    let nonce = format!("{:x}", hasher.finalize());

    // Set token expiration to 48 hours from now.
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(2880)) // 2880 minutes = 48 hours
        .expect("valid timestamp")
        .timestamp() as usize;

    // Create JWT claims.
    let claims = Claims {
        sub: user.email.clone(),
        exp: expiration,
        nonce,
        aud: vec!["".to_string()], // Define your audience
        iss: "".to_string(),       // Define your issuer
    };

    // Encode claims into a JWT using HS256 algorithm and the secret key.
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(auth_key.as_ref()),
    )
    .map_err(|e| e.to_string())?;

    Ok(token)
}

/// Hashes a given password using bcrypt with default cost.
pub fn hash_password(password: String) -> Result<String, String> {
    hash(password, DEFAULT_COST).map_err(|e| e.to_string())
}
"#;

pub const GITIGNORE: &str = r#"/target
.env
"#;

pub const ENV: &str = r#"
DATABASE_URL=
DATABASE=
AUTH_KEY=
"#;

pub const ROCKET_CONFIG: &str = r#"
[default]
# Network settings
address = "0.0.0.0"               # Listen on all network interfaces
port = 8000                       # Port number
workers = 16                      # Number of threads for request handling (adjust to number of CPU cores)
keep_alive = 5                    # Keep-alive timeout in seconds
max_blocking = 512                # Maximum number of blocking operations allowed simultaneously
temp_dir = "/tmp"                 # Directory for temporary files
ident = "Rocket"                  # Server identifier in responses

# Logging and debugging
log_level = "normal"            # Logging level: "critical", "normal", "debug"
cli_colors = true                 # Enable CLI colors for local logs

# Security
ip_header = "X-Real-IP"           # Use reverse proxy header for client IP detection (set to "false" if unused)

# Resource limits
[default.limits]
json = 52428800                 # Max size for JSON payloads (10 MB)
form = 2097152                    # Max size for form submissions (2 MB)
file = 52428800                   # Max size for uploaded files (50 MB)

# TLS configuration (uncomment and configure for HTTPS)
[default.tls]
certs = "/certs/client_cert.pem" # Path to TLS certificate
key = "/certs/client_key.pem"    # Path to private key

[global]
# Global overrides for all environments
address = "0.0.0.0"
port = 8000

[global.limits]
json = 52428800
form = 2097152
file = 52428800
"#;