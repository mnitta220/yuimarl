use super::validation;
use crate::{
    model,
    pages::{
        login_page::LoginPage, page, project_list_page::ProjectListPage, project_page::ProjectPage,
    },
    AppError,
};
use anyhow::Result;
use axum::{
    extract::Query,
    extract::{Form, Path},
    response::Html,
};
use chrono::DateTime;
use firestore::*;
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Debug, Deserialize)]
pub struct Params {
    id: Option<String>,
    tab: Option<String>,
}

pub async fn get_add(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /project_add");

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
    props.title = Some("プロジェクトを作成".to_string());
    props.action = crate::Action::Create;
    props.project = None;

    let mut member = model::project::ProjectMember::new(session.uid.clone());
    member.name = Some(session.name.clone());
    member.email = Some(session.email.clone());
    member.role = Some(model::project::ProjectRole::Owner as i32);
    props.project_members.clear();
    props.project_members.push(member);

    props.session = Some(session);
    let mut page = ProjectPage::new(props, true, true);

    Ok(Html(page.write()))
}

pub async fn get_list(cookies: Cookies) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /project_list");

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
    props.title = Some("プロジェクト一覧".to_string());

    let (projects, owner_cnt) =
        match model::project::ProjectMember::my_projects(&session, &db).await {
            Ok(projects) => projects,
            Err(e) => {
                return Err(AppError(anyhow::anyhow!(e)));
            }
        };

    props.session = Some(session);
    props.project_members = projects;
    let mut page = ProjectListPage::new(props, owner_cnt);

    Ok(Html(page.write()))
}

pub async fn get(cookies: Cookies, Query(params): Query<Params>) -> Result<Html<String>, AppError> {
    let id = params.id.unwrap_or_default();
    let tab = params.tab.unwrap_or_default();
    tracing::debug!("GET /project id={} tab={}", id, tab);

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
    props.action = crate::Action::Update;
    props.title = Some("プロジェクト".to_string());

    match tab.as_ref() {
        "note" => {
            props.tab = crate::Tab::Note;
        }
        "history" => {
            props.tab = crate::Tab::History;
        }
        _ => {
            props.tab = crate::Tab::Info;
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

    for member in &members {
        if member.uid == session.uid {
            props.project_member = Some(member.clone());
        }
    }

    let mut can_update = false;
    let mut can_delete = false;
    // プロジェクトを更新できるのは、オーナー、管理者
    // プロジェクトを削除できるのは、オーナー
    if let Some(pmem) = &props.project_member {
        if let Some(r) = pmem.role {
            if r == model::project::ProjectRole::Owner as i32 {
                can_update = true;
                can_delete = true;
            }
            if r == model::project::ProjectRole::Administrator as i32 {
                can_update = true;
            }
        }
    }

    props.project_members = members;

    props.session = Some(session);
    let mut page = ProjectPage::new(props, can_update, can_delete);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct ProjectInput {
    pub action: String,
    pub project_name: String,
    pub prefix: String,
    pub members: String,
    pub project_id: String,
    pub timestamp: String,
}

pub async fn post(
    cookies: Cookies,
    Form(input): Form<ProjectInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!(
        "POST /project {}, {}, {}, {}",
        input.action,
        input.project_id,
        input.project_name,
        input.timestamp
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

    let action = match input.action.as_ref() {
        "Create" => crate::Action::Create,
        "Update" => crate::Action::Update,
        "Delete" => crate::Action::Delete,
        _ => {
            return Err(AppError(anyhow::anyhow!(format!(
                "invalid action: {}",
                input.action
            ))));
        }
    };

    let mut project_members = Vec::new();
    let empty_vec: Vec<serde_json::Value> = Vec::new();
    let mem = members["members"].as_array().unwrap_or_else(|| &empty_vec);

    for m in mem {
        let mut member =
            model::project::ProjectMember::new(String::from(m["uid"].as_str().unwrap()));
        member.email = Some(String::from(m["email"].as_str().unwrap()));
        member.name = Some(String::from(m["name"].as_str().unwrap()));
        member.role = Some(m["role"].as_i64().unwrap() as i32);
        project_members.push(member);
    }

    let (validation, _project, member) =
        match validation::project::ProjectValidation::validate_post(
            &input,
            &session,
            action.clone(),
            &db,
        )
        .await
        {
            Ok(v) => v,
            Err(e) => {
                return Err(AppError(e));
            }
        };

    if let Some(v) = validation {
        let mut can_update = false;
        let mut can_delete = false;
        // プロジェクトを更新できるのは、オーナー、管理者
        // プロジェクトを削除できるのは、オーナー
        if let Some(pmem) = &member {
            if let Some(r) = pmem.role {
                if r == model::project::ProjectRole::Owner as i32 {
                    can_update = true;
                    can_delete = true;
                }
                if r == model::project::ProjectRole::Administrator as i32 {
                    can_update = true;
                }
            }
        }
        let mut project = model::project::Project::new();
        project.id = Some(input.project_id.clone());
        project.project_name = Some(project_name);
        project.prefix = Some(input.prefix);
        let t = input.timestamp.parse::<i64>().unwrap_or_default();
        project.updated_at = DateTime::from_timestamp_micros(t);
        props.action = action;
        props.session = Some(session);
        props.project = Some(project);
        props.project_validation = Some(v);
        props.project_members = project_members;
        let mut page = ProjectPage::new(props, can_update, can_delete);
        return Ok(Html(page.write()));
    }

    match action {
        crate::Action::Create => {
            // プロジェクト作成
            match model::project::Project::insert(
                &input,
                &session,
                &mut project_members,
                false,
                &db,
            )
            .await
            {
                Ok(p) => p,
                Err(e) => {
                    return Err(AppError(anyhow::anyhow!(e)));
                }
            };
        }
        crate::Action::Update => {
            // プロジェクト更新
            match model::project::Project::update(&input, &session, &mut project_members, &db).await
            {
                Ok(p) => p,
                Err(e) => {
                    return Err(AppError(anyhow::anyhow!(e)));
                }
            };
        }
        crate::Action::Delete => {
            // プロジェクト削除
            match model::project::Project::delete(&input, &session, &db).await {
                Ok(p) => p,
                Err(e) => {
                    return Err(AppError(anyhow::anyhow!(e)));
                }
            };
        }
        _ => {}
    }

    return super::home::show_home(session, &db).await;
}

#[derive(Deserialize, Debug)]
pub struct NoteInput {
    pub markdown: String,
    pub project_id: String,
}

pub async fn post_note(
    cookies: Cookies,
    Form(input): Form<NoteInput>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("POST /post_note {}", input.markdown,);

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
    //let mut props = page::Props::new(&session.id);
    match model::project::Project::update_note(&input, &session, &db).await {
        Ok(p) => p,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    return super::home::show_home(session, &db).await;
}

pub async fn get_project_select(
    cookies: Cookies,
    Path(id): Path<String>,
) -> Result<Html<String>, AppError> {
    tracing::debug!("GET /project_select/{}", id);

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

    match model::project::ProjectMember::update_current(&session, &id, &db).await {
        Ok(_) => {}
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    }

    return super::home::show_home(session, &db).await;
}
