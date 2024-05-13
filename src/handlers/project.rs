use crate::{
    model,
    pages::{home_page::HomePage, login_page::LoginPage, page, project_page::ProjectPage},
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, response::Html};
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
                    let mut member = model::project::ProjectMember::new();
                    member.uid = Some(s.uid.clone());
                    member.name = Some(s.name.clone());
                    member.email = Some(s.email.clone());
                    member.role = Some(model::project::ProjectRole::Owner as i32);
                    props.members.push(member);

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
    pub prefix: String,
    pub members: String,
}

pub async fn post_project(
    cookies: Cookies,
    Form(input): Form<ProjectInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("POST /project {}, {}", input.project_name, input.members);

    let project_name = input.project_name.trim().to_string();
    let members: serde_json::Value = match serde_json::from_str(&input.members) {
        Ok(m) => m,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

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
        let mut project = model::project::Project::new();
        project.project_name = Some(project_name);
        project.prefix = Some(input.prefix);
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
        let mut project = model::project::Project::new();
        project.project_name = Some(project_name);
        project.prefix = Some(input.prefix);
        let validation = model::project::ProjectValidation {
            project_name: Some("同じ名前のプロジェクトが存在します".to_string()),
        };
        props.session = Some(session);
        props.project = Some(project);
        props.project_validation = Some(validation);
        let mut page = ProjectPage::new(props);
        return Ok(Html(page.write()));
    }

    let (prj, project_members) =
        match model::project::Project::insert(&input, &session, members, &db).await {
            Ok(p) => p,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    props.session = Some(session);
    props.project = Some(prj);
    if project_members.len() > 0 {
        if let Some(member) = project_members.get(0) {
            props.member = Some(member.clone());
        }
    }
    props.members = project_members;

    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}
