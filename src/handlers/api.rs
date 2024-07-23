use crate::model;
use axum::extract::{Form, Path};
use firestore::*;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

#[derive(Serialize, Deserialize, Debug)]
pub struct FirebaseConfig {
    pub api_key: String,
    pub auth_domain: String,
    pub project_id: String,
    pub storage_bucket: String,
    pub messaging_sender_id: String,
    pub app_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UserByEmailInput {
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct UserByNameInput {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResult {
    pub result: String,
    pub users: Vec<model::user::UserSub>,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectMemberInput {
    pub project_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberResult {
    pub result: String,
    pub members: Vec<model::project::ProjectMember>,
    pub message: String,
}

pub async fn firebase_config() -> String {
    tracing::debug!("GET /firebase_config");

    let config = FirebaseConfig {
        api_key: crate::API_KEY.get().unwrap().clone(),
        auth_domain: crate::AUTH_DOMAIN.get().unwrap().clone(),
        project_id: crate::GOOGLE_PROJECT_ID.get().unwrap().clone(),
        storage_bucket: crate::STORAGE_BUCKET.get().unwrap().clone(),
        messaging_sender_id: crate::MESSAGING_SENDER_ID.get().unwrap().clone(),
        app_id: crate::APP_ID.get().unwrap().clone(),
    };

    let buf = match serde_json::to_string(&config) {
        Ok(r) => r,
        Err(_) => "NG".to_string(),
    };

    buf
}

/// ユーザーをメールアドレスで検索する
pub async fn user_by_email(Form(input): Form<UserByEmailInput>) -> String {
    tracing::debug!("GET /user_by_email");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            let result = UserResult {
                result: "NG".to_string(),
                users: Vec::new(),
                message: format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let users = match model::user::User::search_by_email(&input.email, &db).await {
        Ok(u) => u,
        Err(e) => {
            let result = UserResult {
                result: "NG".to_string(),
                users: Vec::new(),
                message: format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(e) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let result = UserResult {
        result: "OK".to_string(),
        users: users,
        message: "".to_string(),
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
    };

    buf
}

/// ユーザーを名前で検索する
pub async fn user_by_name(Form(input): Form<UserByNameInput>) -> String {
    tracing::debug!("GET /user_by_name");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            let result = UserResult {
                result: "NG".to_string(),
                users: Vec::new(),
                message: format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let users = match model::user::User::search_by_name(&input.name, &db).await {
        Ok(u) => u,
        Err(e) => {
            let result = UserResult {
                result: "NG".to_string(),
                users: Vec::new(),
                message: format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(e) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let result = UserResult {
        result: "OK".to_string(),
        users: users,
        message: "".to_string(),
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("ユーザーの検索に失敗しました。 [{}]", e.to_string()),
    };

    buf
}

/// プロジェクトのメンバーを取得する。
pub async fn project_member(Form(input): Form<ProjectMemberInput>) -> String {
    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            let result = MemberResult {
                result: "NG".to_string(),
                members: Vec::new(),
                message: format!("メンバーの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("メンバーの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let members = match model::project::ProjectMember::members_of_project(
        &input.project_id,
        false,
        &db,
    )
    .await
    {
        Ok(u) => u,
        Err(e) => {
            let result = MemberResult {
                result: "NG".to_string(),
                members: Vec::new(),
                message: format!("メンバーの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(e) => format!("メンバーの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let result = MemberResult {
        result: "OK".to_string(),
        members: members,
        message: "".to_string(),
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("メンバーの検索に失敗しました。 [{}]", e.to_string()),
    };

    buf
}

#[derive(Deserialize, Debug)]
pub struct UpdateGanttInput {
    pub project_id: String,
    pub tickets: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGanttResult {
    pub result: bool,
    pub message: String,
}

/// ガントチャートを更新する。
pub async fn update_gantt(Form(input): Form<UpdateGanttInput>) -> String {
    println!("{}, {}", input.project_id, input.tickets);

    let result = UpdateGanttResult {
        result: false,
        message: "他のユーザーがチケットを更新しため、更新できませんでした。<br>再読み込みを行ってください。".to_string(),
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
    };

    buf
}

#[derive(Deserialize, Debug)]
pub struct TicketByIdDispInput {
    pub project_id: String,
    pub id_disp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TicketByIdDispResult {
    pub result: bool,
    pub ticket: Option<model::ticket::Ticket>,
    pub message: String,
}

/// チケットをid_dispで検索する
pub async fn ticket_by_id_disp(Form(input): Form<TicketByIdDispInput>) -> String {
    tracing::debug!(
        "POST /ticket_by_id_disp: {}, {}",
        &input.project_id,
        &input.id_disp
    );

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            let result = TicketByIdDispResult {
                result: false,
                ticket: None,
                message: format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let ticket = match model::ticket::Ticket::search_by_id_disp(
        &input.project_id,
        &input.id_disp,
        &db,
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            let result = TicketByIdDispResult {
                result: false,
                ticket: None,
                message: format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(e) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let result = match ticket {
        Some(t) => TicketByIdDispResult {
            result: true,
            ticket: Some(t),
            message: "".to_string(),
        },
        None => TicketByIdDispResult {
            result: false,
            ticket: None,
            message: "該当のチケットは存在しません。".to_string(),
        },
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
    };

    buf
}

#[derive(Deserialize, Debug)]
pub struct TicketColorInput {
    pub ticket_id: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TicketColorResult {
    pub result: bool,
    pub message: String,
}

/// チケットの背景色を設定する
pub async fn ticket_color(cookies: Cookies, Form(input): Form<TicketColorInput>) -> String {
    tracing::debug!("POST /ticket_color: {}, {}", &input.ticket_id, &input.color);

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            let result = TicketColorResult {
                result: false,
                message: format!("背景色の設定に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("背景色の設定に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(e) => {
            let result = TicketColorResult {
                result: false,
                message: format!("背景色の設定に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("背景色の設定に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    if let Err(e) =
        model::ticket::TicketMember::update_color(&input.ticket_id, &input.color, &session, &db)
            .await
    {
        let result = TicketColorResult {
            result: false,
            message: format!("背景色の設定に失敗しました。 [{}]", e.to_string()),
        };

        let buf = match serde_json::to_string(&result) {
            Ok(r) => r,
            Err(_) => format!("背景色の設定に失敗しました。 [{}]", e.to_string()),
        };

        return buf;
    }

    let result = TicketColorResult {
        result: true,
        message: "".to_string(),
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("背景色の設定に失敗しました。 [{}]", e.to_string()),
    };

    buf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TicketWithMemberResult {
    pub result: bool,
    pub ticket: Option<model::ticket::Ticket>,
    pub members: Vec<model::ticket::TicketMember>,
    pub parent: Option<model::ticket::Ticket>,
    pub children: Vec<model::ticket::Ticket>,
    pub message: String,
}

/// ガントチャート画面のチケットダイアログの情報を取得する
pub async fn ticket(cookies: Cookies, Path(id): Path<String>) -> String {
    tracing::info!("GET /ticket: {:?}", &id);

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            let result = TicketWithMemberResult {
                result: false,
                ticket: None,
                members: Vec::new(),
                parent: None,
                children: Vec::new(),
                message: format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(e) => {
            let result = TicketWithMemberResult {
                result: false,
                ticket: None,
                members: Vec::new(),
                parent: None,
                children: Vec::new(),
                message: format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
            };

            let buf = match serde_json::to_string(&result) {
                Ok(r) => r,
                Err(_) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
            };

            return buf;
        }
    };

    let (ticket, _project, project_member, members, parent, children) =
        match model::ticket::Ticket::find_ticket_and_project(&id, &session.uid, &db).await {
            Ok(ticket) => ticket,
            Err(e) => {
                let result = TicketWithMemberResult {
                    result: false,
                    ticket: None,
                    members: Vec::new(),
                    parent: None,
                    children: Vec::new(),
                    message: format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
                };

                let buf = match serde_json::to_string(&result) {
                    Ok(r) => r,
                    Err(_) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
                };

                return buf;
            }
        };

    if project_member.is_none() {
        let result = TicketWithMemberResult {
            result: false,
            ticket: None,
            members: Vec::new(),
            parent: None,
            children: Vec::new(),
            message: "チケットの検索に失敗しました。".to_string(),
        };

        let buf = match serde_json::to_string(&result) {
            Ok(r) => r,
            Err(e) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
        };

        return buf;
    }

    let result = TicketWithMemberResult {
        result: true,
        ticket: ticket,
        members: members,
        parent: parent,
        children: children,
        message: "".to_string(),
    };

    let buf = match serde_json::to_string(&result) {
        Ok(r) => r,
        Err(e) => format!("チケットの検索に失敗しました。 [{}]", e.to_string()),
    };

    buf
}
