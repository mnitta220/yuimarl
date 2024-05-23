use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use pages::page::Props;
use pages::page::Tab;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod components;
mod handlers;
mod model;
mod pages;

static GOOGLE_PROJECT_ID: OnceCell<String> = OnceCell::new();
static API_KEY: OnceCell<String> = OnceCell::new();
static AUTH_DOMAIN: OnceCell<String> = OnceCell::new();
static STORAGE_BUCKET: OnceCell<String> = OnceCell::new();
static MESSAGING_SENDER_ID: OnceCell<String> = OnceCell::new();
static APP_ID: OnceCell<String> = OnceCell::new();
static MEASUREMENT_ID: OnceCell<String> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "yuimarl=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Err(e) = get_environment_values() {
        tracing::error!("failed to get environment values: {:?}", e);
        std::process::exit(0x0100);
    }

    let app = Router::new()
        .route("/", get(handlers::home::get_home))
        .route(
            "/login",
            get(handlers::login::get_login).post(handlers::login::post_login),
        )
        .route("/logout", get(handlers::login::get_logout))
        .route(
            "/agree",
            get(handlers::agreement::get_agree).post(handlers::agreement::post_agree),
        )
        .route("/agree/no", get(handlers::agreement::get_disagree))
        .route(
            "/project",
            get(handlers::project::get).post(handlers::project::post),
        )
        .route("/project_add", get(handlers::project::get_add))
        .route("/project_list", get(handlers::project::get_list))
        .route("/project_note", post(handlers::project::post_note))
        .route("/ticket_add", get(handlers::ticket::get_add))
        //.route("/ticket/add", post(handlers::ticket::post_add_ticket))
        //.route("/ticket/create", post(handlers::ticket::post_create_ticket))
        .route("/contact", get(handlers::contact::get_contact))
        .route("/api/firebaseConfig", get(handlers::api::firebase_config))
        .route("/api/userByEmail", post(handlers::api::user_by_email))
        //.route("/_healthz", get(handlers_bk::health))
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .layer(CookieManagerLayer::new())
                .timeout(Duration::from_secs(10))
                //.layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn get_environment_values() -> Result<()> {
    // get GOOGLE_PROJECT_ID env
    let project_id = match std::env::var("GOOGLE_PROJECT_ID") {
        Ok(project_id) => project_id,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get GOOGLE_PROJECT_ID"));
        }
    };
    // set GOOGLE_PROJECT_ID static
    if let Err(_) = GOOGLE_PROJECT_ID.set(project_id) {
        return Err(anyhow::anyhow!("Failed to set GOOGLE_PROJECT_ID"));
    }

    // get API_KEY env
    let api_key = match std::env::var("API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get API_KEY"));
        }
    };
    // set API_KEY static
    if let Err(_) = API_KEY.set(api_key) {
        return Err(anyhow::anyhow!("Failed to set API_KEY"));
    }

    // get AUTH_DOMAIN env
    let auth_domain = match std::env::var("AUTH_DOMAIN") {
        Ok(auth_domain) => auth_domain,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get AUTH_DOMAIN"));
        }
    };
    // set AUTH_DOMAIN static
    if let Err(_) = AUTH_DOMAIN.set(auth_domain) {
        return Err(anyhow::anyhow!("Failed to set AUTH_DOMAIN"));
    }

    // get STORAGE_BUCKET env
    let storage_bucket = match std::env::var("STORAGE_BUCKET") {
        Ok(storage_bucket) => storage_bucket,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get STORAGE_BUCKET"));
        }
    };
    // set STORAGE_BUCKET static
    if let Err(_) = STORAGE_BUCKET.set(storage_bucket) {
        return Err(anyhow::anyhow!("Failed to set STORAGE_BUCKET"));
    }

    // get MESSAGING_SENDER_ID env
    let messaging_sender_id = match std::env::var("MESSAGING_SENDER_ID") {
        Ok(messaging_sender_id) => messaging_sender_id,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get MESSAGING_SENDER_ID"));
        }
    };
    // set MESSAGING_SENDER_ID static
    if let Err(_) = MESSAGING_SENDER_ID.set(messaging_sender_id) {
        return Err(anyhow::anyhow!("Failed to set MESSAGING_SENDER_ID"));
    }

    // get APP_ID env
    let app_id = match std::env::var("APP_ID") {
        Ok(app_id) => app_id,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get APP_ID"));
        }
    };
    // set APP_ID static
    if let Err(_) = APP_ID.set(app_id) {
        return Err(anyhow::anyhow!("Failed to set APP_ID"));
    }

    // get MEASUREMENT_ID env
    let measurement_id = match std::env::var("MEASUREMENT_ID") {
        Ok(measurement_id) => measurement_id,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to get MEASUREMENT_ID"));
        }
    };
    // set MEASUREMENT_ID static
    if let Err(_) = MEASUREMENT_ID.set(measurement_id) {
        return Err(anyhow::anyhow!("Failed to set MEASUREMENT_ID"));
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Read,
    Create,
    Update,
    Delete,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Read => "Read".to_string(),
            Action::Create => "Create".to_string(),
            Action::Update => "Update".to_string(),
            Action::Delete => "Delete".to_string(),
        }
    }
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
