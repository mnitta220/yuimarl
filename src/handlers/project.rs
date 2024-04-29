use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page, project_page::ProjectPage},
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
use chrono::Utc;
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

pub async fn get_add_project(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /project/add");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let buf = match super::session_info(cookies, true) {
        Ok(session_id) => {
            let mut props = page::Props::new(&session_id);
            let session = model::session::Session::find(&session_id, &db).await?;

            match session {
                Some(s) => {
                    props.session = Some(s);
                    let mut page = ProjectPage::new(props);
                    page.write()
                }
                None => LoginPage::write(),
            }
        }
        Err(_) => LoginPage::write(),
    };

    Ok(Html(buf))
}

#[derive(Deserialize, Debug)]
pub struct ProjectInput {
    pub project_name: String,
    //pub language: String,
    pub prefix: String,
}

pub async fn post_project(
    cookies: Cookies,
    Form(input): Form<ProjectInput>,
) -> Result<Html<String>, AppError> {
    //tracing::debug!("POST /project {}, {}", input.project_name, input.language);
    let project_name = input.project_name.trim().to_string();

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session_id = match super::session_info(cookies, true) {
        Ok(session_id) => session_id,
        Err(_) => {
            return Ok(Html(LoginPage::write()));
        }
    };

    let mut props = page::Props::new(&session_id);
    let session = model::session::Session::find(&session_id, &db).await?;

    let session = match session {
        Some(s) => s,
        None => {
            return Ok(Html(LoginPage::write()));
        }
    };

    if project_name.len() == 0 {
        let project = model::project::Project {
            id: "".to_string(),
            project_name: project_name,
            language: "ja".to_string(),
            owner: "".to_string(),
            prefix: "".to_string(),
            ticket_limit: model::project::TICKET_LIMIT_DEFAULT,
            ticket_number: 0,
            created_at: Utc::now(),
        };
        let validation = model::project::ProjectValidation {
            project_name: Some("入力してください".to_string()),
        };
        props.session = Some(session);
        props.project = Some(project);
        props.project_validation = Some(validation);
        let mut page = ProjectPage::new(props);
        return Ok(Html(page.write()));
    }

    let p = match model::project::Project::find_by_owner_and_name(&session.uid, &project_name, &db)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    if p.len() > 0 {
        let project = model::project::Project {
            id: "".to_string(),
            project_name: project_name,
            language: "ja".to_string(),
            owner: "".to_string(),
            prefix: "".to_string(),
            ticket_limit: model::project::TICKET_LIMIT_DEFAULT,
            ticket_number: 0,
            created_at: Utc::now(),
        };
        let validation = model::project::ProjectValidation {
            project_name: Some("同じ名前のプロジェクトが存在します".to_string()),
        };
        props.session = Some(session);
        props.project = Some(project);
        props.project_validation = Some(validation);
        let mut page = ProjectPage::new(props);
        return Ok(Html(page.write()));
    }

    if let Err(e) = model::project::Project::insert(&input, &session, &db).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    let projects = model::project::Project::my_projects(&session, &db).await?;
    props.projects = projects;
    props.session = Some(session);

    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct MemberAddInput {
    pub add_members: String,
}

pub async fn post_member_add(
    cookies: Cookies,
    Form(input): Form<MemberAddInput>,
) -> Result<Html<String>, AppError> {
    tracing::info!("POST /member/add, {}", input.add_members);

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session_id = match super::session_info(cookies, true) {
        Ok(session_id) => session_id,
        Err(_) => {
            return Ok(Html(LoginPage::write()));
        }
    };

    let mut props = page::Props::new(&session_id);
    let session = model::session::Session::find(&session_id, &db).await?;

    let session = match session {
        Some(s) => s,
        None => {
            return Ok(Html(LoginPage::write()));
        }
    };

    let project_id = match &session.project_id {
        Some(p) => p,
        None => {
            return Ok(Html(LoginPage::write()));
        }
    };

    let r = match model::project::ProjectMember::add_project_member(
        project_id,
        &input.add_members,
        &db,
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    /*
    let v: Vec<&str> = input.add_members.split(',').collect();
    let mut i = 0;
    let mut uid: &str;
    let mut role: &str;

    loop {
        if i >= v.len() {
            break;
        }
        uid = v[i];
        i += 1;
        if i >= v.len() {
            break;
        }
        role = v[i];
        i += 1;
        tracing::info!("uid={} role={}", uid, role);
    }
    */

    props.session = Some(session);
    let mut page = ProjectPage::new(props);

    Ok(Html(page.write()))
}