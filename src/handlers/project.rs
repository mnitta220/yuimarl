use crate::{
    model,
    pages::{
        home_page::HomePage, login_page::LoginPage, page, project_list_page::ProjectListPage,
        project_page::ProjectPage,
    },
    AppError,
};
use anyhow::Result;
use axum::{extract::Form, extract::Query, response::Html};
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
    tab: Option<String>,
}

pub async fn get_add_project(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /project/add");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session.id);
    props.project = None;

    let mut member = model::project::ProjectMember::new(session.uid.clone());
    //member.uid = Some(session.uid.clone());
    member.name = Some(session.name.clone());
    member.email = Some(session.email.clone());
    member.role = Some(model::project::ProjectRole::Owner as i32);
    props.members.clear();
    props.members.push(member);

    props.session = Some(session);
    let mut page = ProjectPage::new(props);

    Ok(Html(page.write()))
}

pub async fn get_project_list(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /project/list");

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session.id);

    let projects = match model::project::ProjectMember::my_projects(&session, &db).await {
        Ok(projects) => projects,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    props.session = Some(session);
    props.members = projects;
    let mut page = ProjectListPage::new(props);

    Ok(Html(page.write()))
}

pub async fn get_project(
    cookies: Cookies,
    Query(params): Query<Params>,
) -> Result<Html<String>, AppError> {
    //tracing::info!("GET /project {} {}", id, params.tab.unwrap_or_default());
    let id = params.id.unwrap_or_default();
    let tab = params.tab.unwrap_or_default();
    tracing::info!("GET /project id={} tab={}", id, tab);

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session.id);

    match tab.as_ref() {
        "note" => {
            props.project_tab = crate::ProjectTab::Note;
        }
        "history" => {
            props.project_tab = crate::ProjectTab::History;
        }
        _ => {
            props.project_tab = crate::ProjectTab::Info;
        }
    }

    let project = match model::project::Project::find(&id, &db).await {
        Ok(project) => project,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };
    props.project = project;

    let members = match model::project::ProjectMember::members_of_project(&id, false, &db).await {
        Ok(members) => members,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    props.members = members;

    props.session = Some(session);
    let mut page = ProjectPage::new(props);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct ProjectInput {
    pub project_name: String,
    pub prefix: String,
    pub members: String,
    pub project_id: String,
}

pub async fn post_project(
    cookies: Cookies,
    Form(input): Form<ProjectInput>,
) -> Result<Html<String>, AppError> {
    tracing::info!(
        "POST /project {}, {}, {}",
        input.project_id,
        input.project_name,
        input.members
    );

    let project_name = input.project_name.trim().to_string();
    let members = format!(r#"{{"members":{}}}"#, input.members);
    let members: serde_json::Value = match serde_json::from_str(&members) {
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

    let session = match super::get_session_info(cookies, true, &db).await {
        Ok(session_id) => session_id,
        Err(_) => return Ok(Html(LoginPage::write())),
    };
    let mut props = page::Props::new(&session.id);
    let mut project_members = Vec::new();
    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let mem = members["members"].as_array().unwrap_or_else(|| &empty_vec);

    for m in mem {
        let mut member =
            model::project::ProjectMember::new(String::from(m["uid"].as_str().unwrap()));
        //member.uid = Some(String::from(m["uid"].as_str().unwrap()));
        member.email = Some(String::from(m["email"].as_str().unwrap()));
        member.name = Some(String::from(m["name"].as_str().unwrap()));
        member.role = Some(m["role"].as_i64().unwrap() as i32);
        project_members.push(member);
    }

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
        props.members = project_members;
        let mut page = ProjectPage::new(props);
        return Ok(Html(page.write()));
    }

    let projects =
        match model::project::Project::find_by_owner_and_name(&session.uid, &project_name, &db)
            .await
        {
            Ok(p) => p,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    for prj in projects {
        if prj.id.unwrap() == input.project_id {
            continue;
        }

        let mut project = model::project::Project::new();
        project.id = Some(input.project_id.clone());
        project.project_name = Some(project_name);
        project.prefix = Some(input.prefix);
        let validation = model::project::ProjectValidation {
            project_name: Some("同じ名前のプロジェクトが存在します".to_string()),
        };
        props.session = Some(session);
        props.project = Some(project);
        props.project_validation = Some(validation);
        props.members = project_members;
        let mut page = ProjectPage::new(props);
        return Ok(Html(page.write()));
    }

    if input.project_id.len() == 0 {
        let prj = match model::project::Project::insert(&input, &session, &mut project_members, &db)
            .await
        {
            Ok(p) => p,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

        props.session = Some(session);
        props.project = Some(prj);
    } else {
        let prj = match model::project::Project::update(&input, &mut project_members, &db).await {
            Ok(p) => p,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

        props.session = Some(session);
        props.project = Some(prj);
    }

    if project_members.len() > 0 {
        if let Some(member) = project_members.get(0) {
            props.member = Some(member.clone());
        }
    }
    props.members = project_members;

    let mut page = HomePage::new(props);

    Ok(Html(page.write()))
}
