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
use pages::page::{Props, Screen, Tab};
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
static DB_CHECK_PASSWORD: OnceCell<String> = OnceCell::new();
static NOTICE_PASSWORD: OnceCell<String> = OnceCell::new();
static E2E_TEST_PASSWORD: OnceCell<String> = OnceCell::new();
static HOLIDAYS: OnceCell<Vec<&str>> = OnceCell::new();

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
            get(handlers::agreement::get).post(handlers::agreement::post),
        )
        .route("/agree/no", get(handlers::agreement::get_disagree))
        .route("/user_name", post(handlers::user_name::post))
        .route(
            "/project",
            get(handlers::project::get).post(handlers::project::post),
        )
        .route("/project_add", get(handlers::project::get_add))
        .route("/project_list", get(handlers::project::get_list))
        .route("/project_note", post(handlers::project::post_note))
        .route(
            "/project_select/:id",
            get(handlers::project::get_project_select),
        )
        .route("/ticket_add", get(handlers::ticket::get_add))
        .route(
            "/ticket",
            get(handlers::ticket::get).post(handlers::ticket::post),
        )
        .route("/ticket_note", post(handlers::ticket::post_note))
        .route(
            "/ticket_list",
            get(handlers::ticket_list::get_list).post(handlers::ticket_list::post_list),
        )
        .route("/post_comment", post(handlers::ticket::post_comment))
        .route("/post_memo", post(handlers::user_memo::post))
        .route("/news_del/:id", get(handlers::news::del_news))
        .route("/contact", get(handlers::contact::get_contact))
        .route("/api/firebaseConfig", get(handlers::api::firebase_config))
        .route("/api/userByEmail", post(handlers::api::user_by_email))
        .route("/api/userByName", post(handlers::api::user_by_name))
        .route("/api/projectMember", post(handlers::api::project_member))
        .route("/api/ganttUpdate", post(handlers::api::update_gantt))
        .route(
            "/api/ticketByIdDisp",
            post(handlers::api::ticket_by_id_disp),
        )
        .route("/api/ticketColor", post(handlers::api::ticket_color))
        .route("/api/ticket/:id", get(handlers::api::ticket))
        .route(
            "/db_check",
            get(handlers::db_check::get).post(handlers::db_check::post),
        )
        .route(
            "/notice_add",
            get(handlers::notice::get_add).post(handlers::notice::post_add),
        )
        .route(
            "/notice_del",
            get(handlers::notice::get_del).post(handlers::notice::post_del),
        )
        .route(
            "/e2e_test",
            get(handlers::e2e_test::get).post(handlers::e2e_test::post),
        )
        .route("/health", get(handlers::health))
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

    // get DB_CHECK_PASSWORD env
    let db_check_password = match std::env::var("DB_CHECK_PASSWORD") {
        Ok(db_check_password) => db_check_password,
        Err(_) => "".to_string(),
    };
    // set DB_CHECK_PASSWORD static
    if let Err(_) = DB_CHECK_PASSWORD.set(db_check_password) {
        return Err(anyhow::anyhow!("Failed to set DB_CHECK_PASSWORD"));
    }

    // get NOTICE_PASSWORD env
    let notice_password = match std::env::var("NOTICE_PASSWORD") {
        Ok(notice_password) => notice_password,
        Err(_) => "".to_string(),
    };
    // set DB_CHECK_PASSWORD static
    if let Err(_) = NOTICE_PASSWORD.set(notice_password) {
        return Err(anyhow::anyhow!("Failed to set NOTICE_PASSWORD"));
    }

    // get E2E_TEST_PASSWORD env
    let e2e_test_password = match std::env::var("E2E_TEST_PASSWORD") {
        Ok(e2e_test_password) => e2e_test_password,
        Err(_) => "".to_string(),
    };
    // set E2E_TEST_PASSWORD static
    if let Err(_) = E2E_TEST_PASSWORD.set(e2e_test_password) {
        return Err(anyhow::anyhow!("Failed to set E2E_TEST_PASSWORD"));
    }

    // set HOLIDAYS static (日本の祝日)
    if let Err(_) = HOLIDAYS.set(vec![
        "2020-01-01", // 元日
        "2020-01-13", // 成人の日
        "2020-02-11", // 建国記念の日
        "2020-02-23", // 天皇誕生日
        "2020-02-24", // 振替休日
        "2020-03-20", // 春分の日
        "2020-04-29", // 昭和の日
        "2020-05-03", // 憲法記念日
        "2020-05-04", // みどりの日
        "2020-05-05", // こどもの日
        "2020-05-06", // 振替休日
        "2020-07-23", // 海の日
        "2020-07-24", // スポーツの日
        "2020-08-10", // 山の日
        "2020-09-21", // 敬老の日
        "2020-09-22", // 秋分の日
        "2020-11-03", // 文化の日
        "2020-11-23", // 勤労感謝の日
        "2021-01-01", // 元日
        "2021-01-11", // 成人の日
        "2021-02-11", // 建国記念の日
        "2021-02-23", // 天皇誕生日
        "2021-03-20", // 春分の日
        "2021-04-29", // 昭和の日
        "2021-05-03", // 憲法記念日
        "2021-05-04", // みどりの日
        "2021-05-05", // こどもの日
        "2021-07-19", // 海の日
        "2021-08-11", // 山の日
        "2021-09-20", // 敬老の日
        "2021-09-23", // 秋分の日
        "2021-10-11", // スポーツの日
        "2021-11-03", // 文化の日
        "2021-11-23", // 勤労感謝の日
        "2022-01-01", // 元日
        "2022-01-10", // 成人の日
        "2022-02-11", // 建国記念の日
        "2022-02-23", // 天皇誕生日
        "2022-03-21", // 春分の日
        "2022-04-29", // 昭和の日
        "2022-05-03", // 憲法記念日
        "2022-05-04", // みどりの日
        "2022-05-05", // こどもの日
        "2022-07-18", // 海の日
        "2022-08-11", // 山の日
        "2022-09-19", // 敬老の日
        "2022-09-23", // 秋分の日
        "2022-10-10", // スポーツの日
        "2022-11-03", // 文化の日
        "2022-11-23", // 勤労感謝の日
        "2023-01-01", // 元日
        "2023-01-02", // 振替休日
        "2023-01-09", // 成人の日
        "2023-02-11", // 建国記念の日
        "2023-02-23", // 天皇誕生日
        "2023-03-21", // 春分の日
        "2023-04-29", // 昭和の日
        "2023-05-03", // 憲法記念日
        "2023-05-04", // みどりの日
        "2023-05-05", // こどもの日
        "2023-07-17", // 海の日
        "2023-08-11", // 山の日
        "2023-09-18", // 敬老の日
        "2023-09-23", // 秋分の日
        "2023-10-09", // スポーツの日
        "2023-11-03", // 文化の日
        "2023-11-23", // 勤労感謝の日
        "2024-01-01", // 元日
        "2024-01-08", // 成人の日
        "2024-02-11", // 建国記念の日
        "2024-02-12", // 振替休日
        "2024-02-23", // 天皇誕生日
        "2024-03-20", // 春分の日
        "2024-04-29", // 昭和の日
        "2024-05-03", // 憲法記念日
        "2024-05-04", // みどりの日
        "2024-05-05", // こどもの日
        "2024-05-06", // 振替休日
        "2024-07-15", // 海の日
        "2024-08-11", // 山の日
        "2024-08-12", // 振替休日
        "2024-09-16", // 敬老の日
        "2024-09-22", // 秋分の日
        "2024-09-23", // 振替休日
        "2024-10-14", // スポーツの日
        "2024-11-03", // 文化の日
        "2024-11-04", // 振替休日
        "2024-11-23", // 勤労感謝の日
        "2025-01-01", // 元日
        "2025-01-13", // 成人の日
        "2025-02-11", // 建国記念の日
        "2025-02-23", // 天皇誕生日
        "2024-02-24", // 振替休日
        "2025-03-20", // 春分の日
        "2025-04-29", // 昭和の日
        "2025-05-03", // 憲法記念日
        "2025-05-04", // みどりの日
        "2025-05-05", // こどもの日
        "2025-05-06", // 振替休日
        "2025-07-21", // 海の日
        "2025-08-11", // 山の日
        "2025-09-15", // 敬老の日
        "2025-09-23", // 秋分の日
        "2025-10-13", // スポーツの日
        "2025-11-03", // 文化の日
        "2025-11-23", // 勤労感謝の日
        "2025-11-24", // 振替休日
        "2026-01-01", // 元日
        "2026-01-12", // 成人の日
        "2026-02-11", // 建国記念の日
        "2026-02-23", // 天皇誕生日
        "2026-03-20", // 春分の日
        "2026-04-29", // 昭和の日
        "2026-05-03", // 憲法記念日
        "2026-05-04", // みどりの日
        "2026-05-05", // こどもの日
        "2026-05-06", // 振替休日
        "2026-07-20", // 海の日
        "2026-08-11", // 山の日
        "2026-09-21", // 敬老の日
        "2026-09-22", // 国民の休日
        "2026-09-23", // 秋分の日
        "2026-10-12", // スポーツの日
        "2026-11-03", // 文化の日
        "2026-11-23", // 勤労感謝の日
        "2027-01-01", // 元日
        "2027-01-11", // 成人の日
        "2027-02-11", // 建国記念の日
        "2027-02-23", // 天皇誕生日
        "2027-03-21", // 春分の日
        "2027-03-22", // 振替休日
        "2027-04-29", // 昭和の日
        "2027-05-03", // 憲法記念日
        "2027-05-04", // みどりの日
        "2027-05-05", // こどもの日
        "2027-07-19", // 海の日
        "2027-08-11", // 山の日
        "2027-09-20", // 敬老の日
        "2027-09-23", // 秋分の日
        "2027-10-11", // スポーツの日
        "2027-11-03", // 文化の日
        "2027-11-23", // 勤労感謝の日
        "2028-01-01", // 元日
        "2028-01-10", // 成人の日
        "2028-02-11", // 建国記念の日
        "2028-02-23", // 天皇誕生日
        "2028-03-20", // 春分の日
        "2028-04-29", // 昭和の日
        "2028-05-03", // 憲法記念日
        "2028-05-04", // みどりの日
        "2028-05-05", // こどもの日
        "2028-07-17", // 海の日
        "2028-08-11", // 山の日
        "2028-09-18", // 敬老の日
        "2028-09-22", // 秋分の日
        "2028-10-09", // スポーツの日
        "2028-11-03", // 文化の日
        "2028-11-23", // 勤労感謝の日
        "2029-01-01", // 元日
        "2029-01-08", // 成人の日
        "2029-02-11", // 建国記念の日
        "2029-02-12", // 振替休日
        "2029-02-23", // 天皇誕生日
        "2029-03-20", // 春分の日
        "2029-04-29", // 昭和の日
        "2029-04-30", // 振替休日
        "2029-05-03", // 憲法記念日
        "2029-05-04", // みどりの日
        "2029-05-05", // こどもの日
        "2029-07-16", // 海の日
        "2029-08-11", // 山の日
        "2029-09-17", // 敬老の日
        "2029-09-23", // 秋分の日
        "2029-09-24", // 振替休日
        "2029-10-08", // スポーツの日
        "2029-11-03", // 文化の日
        "2029-11-23", // 勤労感謝の日
        "2030-01-01", // 元日
        "2030-01-14", // 成人の日
        "2030-02-11", // 建国記念の日
        "2030-02-23", // 天皇誕生日
        "2030-03-20", // 春分の日
        "2030-04-29", // 昭和の日
        "2030-05-03", // 憲法記念日
        "2030-05-04", // みどりの日
        "2030-05-05", // こどもの日
        "2030-05-06", // 振替休日
        "2030-07-15", // 海の日
        "2030-08-11", // 山の日
        "2030-08-12", // 振替休日
        "2030-09-16", // 敬老の日
        "2030-09-23", // 秋分の日
        "2030-10-14", // スポーツの日
        "2030-11-03", // 文化の日
        "2030-11-04", // 振替休日
        "2030-11-23", // 勤労感謝の日
    ]) {
        return Err(anyhow::anyhow!("Failed to set HOLIDAYS"));
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Read,
    Create,
    Update,
    Delete,
    Withdraw,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Read => "Read".to_string(),
            Action::Create => "Create".to_string(),
            Action::Update => "Update".to_string(),
            Action::Delete => "Delete".to_string(),
            Action::Withdraw => "Withdraw".to_string(),
        }
    }
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        const INDEX_ERROR_MSG: &str = "The query requires an index. You can create it here: ";

        let mut msg = format!("エラーが発生しました。:\r\n\r\n{}", self.0);
        let f = msg.find(INDEX_ERROR_MSG);
        if let Some(f) = f {
            let msg2 = &msg[f + INDEX_ERROR_MSG.len()..];
            let f = msg2.find(r#"""#);
            if let Some(f) = f {
                msg = format!("データベースのインデックスを作成する必要があります。次の URL にアクセスして、インデックスを作成してください。\r\n\r\n{}", &msg2[..f]);
            }
        }
        println!("{:?}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
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
