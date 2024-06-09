use crate::{handlers, model};
use anyhow::Result;
use firestore::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbCheckValidation {
    pub info: Option<String>,
    pub db_check_password: Option<String>,
    pub result: bool,
}

impl DbCheckValidation {
    pub fn new() -> Self {
        Self {
            info: None,
            db_check_password: None,
            result: false,
        }
    }

    pub async fn validate_post(
        input: &handlers::db_check::DbCheckInput,
        session: &model::session::Session,
        db: &FirestoreDb,
    ) -> Result<Option<Self>> {
        let db_check_password = match crate::DB_CHECK_PASSWORD.get() {
            Some(password) => password.to_string(),
            None => "".to_string(),
        };

        if db_check_password.len() == 0 {
            let mut validation = Self::new();
            validation.info = Some("DB_CHECK_PASSWORD 環境変数が設定されていません".to_string());
            return Ok(Some(validation));
        }

        if input.db_check_password != db_check_password {
            let mut validation = Self::new();
            validation.db_check_password =
                Some("DB_CHECK_PASSWORD が正しくありません。".to_string());
            return Ok(Some(validation));
        }

        let project_input = super::super::project::ProjectInput {
            action: String::from("Create"),
            project_name: String::from("データベースチェック"),
            prefix: String::from("ch"),
            members: String::from(""),
            project_id: String::from(""),
            timestamp: String::from(""),
        };

        let mut project_members = Vec::new();
        let mut member = model::project::ProjectMember::new("", "", &session.uid);
        member.email = Some(String::from("db_check@mail.com"));
        member.name = Some(String::from("データベースチェック"));
        member.role = Some(1);
        project_members.push(member);

        // プロジェクト作成
        let prj = match model::project::Project::insert(
            &project_input,
            &session,
            &mut project_members,
            true,
            &db,
        )
        .await
        {
            Ok(p) => p,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let ticket_input = super::super::ticket::TicketInput {
            action: String::from("Create"),
            name: String::from("データベースチェック"),
            description: String::from(
                "データベースチェックで作成されたチケットです。削除してください。",
            ),
            members: String::from(""),
            start_date: String::from(""),
            end_date: String::from(""),
            progress: String::from("0"),
            priority: String::from("0"),
            parent: String::from("0"),
            deliverables: String::from(""),
            project_id: String::from(&prj.id),
            ticket_id: String::from(""),
            timestamp: String::from(""),
        };

        let mut ticket_members = Vec::new();
        let mut member = model::ticket::TicketMember::new("", "", &prj.id, &session.uid);
        member.seq = 0;
        ticket_members.push(member);

        let ticket = match model::ticket::Ticket::insert(
            &ticket_input,
            &session,
            &prj,
            &ticket_members,
            &db,
        )
        .await
        {
            Ok(t) => t,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::project::Project::current_project_and_tickets(&session, &db).await {
            Ok(u) => u,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::project::Project::find_by_owner_and_name(
            &session.uid,
            &String::from("データベースチェック"),
            &db,
        )
        .await
        {
            Ok(u) => u,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::project::ProjectMember::members_of_project(&prj.id, false, &db).await {
            Ok(u) => u,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::project::ProjectMember::members_of_project(&prj.id, true, &db).await {
            Ok(u) => u,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::project::ProjectMember::find(&prj.id, &session.uid, &db).await {
            Ok(u) => u,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::project::ProjectMember::my_projects(&session, &db).await {
            Ok(projects) => projects,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::user::User::search_by_email(&String::from("db_check@mail.com"), &db).await {
            Ok(u) => u,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::ticket::Ticket::find_ticket_and_project(&ticket.id, &session.uid, &db).await {
            Ok(ticket) => ticket,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        let input = super::super::ticket::TicketListInput {
            ticketid: String::from(""),
            ticketname: String::from(""),
            parentid: String::from(""),
            finished: None,
        };

        match model::ticket::Ticket::search_list(&prj.id, &input, &db).await {
            Ok(tickets) => tickets,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        match model::news::News::get_news_list(&session.uid, db).await {
            Ok(news) => news,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        if let Err(e) = model::project::Project::delete_db_check(&db).await {
            return Err(anyhow::anyhow!(e));
        }

        let mut validation = Self::new();
        validation.result = true;
        Ok(Some(validation))
    }
}
