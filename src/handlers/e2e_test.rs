use super::validation;
use crate::{
    model,
    pages::{e2e_test::E2eTestPage, page},
    AppError,
};
use anyhow::Result;
use axum::{
    extract::Form,
    response::{Html, Redirect},
};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub async fn get() -> Result<Html<String>, AppError> {
    tracing::debug!("GET /e2e_test");

    let mut props = page::Props::new();
    props.title = Some(String::from("E2Eテスト"));
    let mut page = E2eTestPage::new(props, None, None);

    Ok(Html(page.write()))
}

#[derive(Deserialize, Debug)]
pub struct E2eTestInput {
    pub e2e_test_password: String,
    pub user: String,
}

pub async fn post(cookies: Cookies, Form(input): Form<E2eTestInput>) -> Result<Redirect, AppError> {
    tracing::debug!(
        "POST /e2e_test, e2e_test_password: {}, user: {}",
        input.e2e_test_password,
        input.user
    );

    let db = match FirestoreDb::new(crate::GOOGLE_PROJECT_ID.get().unwrap()).await {
        Ok(db) => db,
        Err(e) => {
            return Err(AppError(anyhow::anyhow!(e)));
        }
    };

    let v = match validation::e2e_test::E2eTestValidation::validate_post(&input).await {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError(e));
        }
    };

    if let Some(validation) = v.as_ref() {
        if validation.result == false {
            return Ok(Redirect::to("/e2e_test"));
        }
    }

    let user = match get_e2e_user(&input.user, &db).await {
        Ok(u) => u,
        Err(e) => {
            return Err(AppError(e));
        }
    };

    let session_id = Uuid::now_v7().to_string();
    let session = model::session::Session {
        id: session_id.to_string(),
        uid: user.uid.clone(),
        name: user.name.clone(),
        email: user.email.clone(),
        photo_url: "".to_string(),
        e2e_test: Some(true),
        created_at: chrono::Utc::now(),
    };

    let mut c = Cookie::new(super::COOKIE_SESSION_ID, session_id);
    let mut expire = time::OffsetDateTime::now_utc();
    expire += time::Duration::weeks(52);
    c.set_expires(expire);
    cookies.add(c);

    if let Err(e) = model::session::Session::upsert(&session, &db).await {
        return Err(AppError(anyhow::anyhow!(e)));
    }

    Ok(Redirect::to("/"))
}

pub async fn delete_e2e_test_data(db: &FirestoreDb) -> Result<()> {
    let mut transaction = match db.begin_transaction().await {
        Ok(t) => t,
        Err(e) => {
            return Err(anyhow::anyhow!(e.to_string()));
        }
    };

    let sessions_stream: BoxStream<FirestoreResult<model::session::Session>> = match db
        .fluent()
        .select()
        .fields(paths!(model::session::Session::{id, uid, name, email, photo_url, e2e_test, created_at}))
        .from(model::session::COLLECTION_NAME)
        .filter(|q| q.for_all([q.field(path!(model::session::Session::e2e_test)).eq(true)]))
        .obj()
        .stream_query_with_errors()
        .await
    {
        Ok(s) => s,
        Err(e) => {
            return Err(anyhow::anyhow!(e.to_string()));
        }
    };

    let sessions: Vec<model::session::Session> = match sessions_stream.try_collect().await {
        Ok(s) => s,
        Err(e) => {
            return Err(anyhow::anyhow!(e.to_string()));
        }
    };

    for session in sessions {
        if let Err(e) = db
            .fluent()
            .delete()
            .from(model::session::COLLECTION_NAME)
            .document_id(&session.id)
            .add_to_transaction(&mut transaction)
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }
    }

    let users_stream: BoxStream<FirestoreResult<model::user::User>> = match db
        .fluent()
        .select()
        .fields(
            paths!(model::user::User::{uid, email, name, status, e2e_test, created_at, last_login}),
        )
        .from(model::user::COLLECTION_NAME)
        .filter(|q| q.for_all([q.field(path!(model::user::User::e2e_test)).eq(true)]))
        .obj()
        .stream_query_with_errors()
        .await
    {
        Ok(s) => s,
        Err(e) => {
            return Err(anyhow::anyhow!(e.to_string()));
        }
    };

    let users: Vec<model::user::User> = match users_stream.try_collect().await {
        Ok(s) => s,
        Err(e) => {
            return Err(anyhow::anyhow!(e.to_string()));
        }
    };

    for user in users {
        let projects_stream: BoxStream<FirestoreResult<model::project::Project>> = match db
            .fluent()
            .select()
            .fields(paths!(model::project::Project::{id, db_check, deleted}))
            .from(model::project::COLLECTION_NAME)
            .filter(|q| {
                q.for_all([q
                    .field(path!(model::project::Project::owner))
                    .eq(user.uid.clone())])
            })
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let projects: Vec<model::project::Project> = match projects_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        for prj in projects {
            let project_members_stream: BoxStream<FirestoreResult<model::project::ProjectMember>> =
                match db
                    .fluent()
                    .select()
                    .fields(paths!(model::project::ProjectMember::{id, project_id, uid}))
                    .from(model::project::COLLECTION_MEMBER)
                    .filter(|q| {
                        q.for_all([q
                            .field(path!(model::project::ProjectMember::project_id))
                            .eq(&prj.id)])
                    })
                    .obj()
                    .stream_query_with_errors()
                    .await
                {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                };

            let project_members: Vec<model::project::ProjectMember> =
                match project_members_stream.try_collect().await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                };

            for member in project_members {
                if let Err(e) = db
                    .fluent()
                    .delete()
                    .from(model::project::COLLECTION_MEMBER)
                    .document_id(&member.id)
                    .add_to_transaction(&mut transaction)
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }

            let tickets_stream: BoxStream<FirestoreResult<model::ticket::Ticket>> = match db
                .fluent()
                .select()
                .fields(paths!(model::ticket::Ticket::{id, project_id, progress, priority}))
                .from(model::ticket::COLLECTION_NAME)
                .filter(|q| {
                    q.for_all([q
                        .field(path!(model::ticket::Ticket::project_id))
                        .eq(&prj.id)])
                })
                .obj()
                .stream_query_with_errors()
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            let tickets: Vec<model::ticket::Ticket> = match tickets_stream.try_collect().await {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            };

            for ticket in tickets {
                let ticket_members_stream: BoxStream<FirestoreResult<model::ticket::TicketMember>> =
                    match db
                        .fluent()
                        .select()
                        .fields(paths!(model::ticket::TicketMember::{id, ticket_id, project_id, uid, seq}))
                        .from(model::ticket::COLLECTION_MEMBER)
                        .filter(|q| q.for_all([q.field(path!(model::ticket::TicketMember::ticket_id)).eq(&ticket.id)]))
                        .obj()
                    .stream_query_with_errors()
                    .await
                {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                };

                let members: Vec<model::ticket::TicketMember> =
                    match ticket_members_stream.try_collect().await {
                        Ok(s) => s,
                        Err(e) => {
                            return Err(anyhow::anyhow!(e.to_string()));
                        }
                    };

                for member in members {
                    if let Err(e) = db
                        .fluent()
                        .delete()
                        .from(model::ticket::COLLECTION_MEMBER)
                        .document_id(&member.id)
                        .add_to_transaction(&mut transaction)
                    {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                }

                if let Err(e) = db
                    .fluent()
                    .delete()
                    .from(model::ticket::COLLECTION_NAME)
                    .document_id(&ticket.id)
                    .add_to_transaction(&mut transaction)
                {
                    return Err(anyhow::anyhow!(e.to_string()));
                }
            }

            if let Err(e) = db
                .fluent()
                .delete()
                .from(model::project::COLLECTION_NAME)
                .document_id(&prj.id)
                .add_to_transaction(&mut transaction)
            {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        }
    }

    if let Err(e) = transaction.commit().await {
        return Err(anyhow::anyhow!(e.to_string()));
    }

    Ok(())
}

pub async fn get_e2e_user(login_user: &str, db: &FirestoreDb) -> Result<model::user::User> {
    // e2eテストユーザーが存在しない場合は作成する
    let yamada_user = match model::user::User::find_or_create_e2e_test_user(
        "山田太郎",
        "taro.yamada@e2e_test.com",
        db,
    )
    .await
    {
        Ok(u) => u,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let iwaki_user = match model::user::User::find_or_create_e2e_test_user(
        "岩鬼正美",
        "masami.iwaki@e2e_test.com",
        db,
    )
    .await
    {
        Ok(u) => u,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let tonoma_user = match model::user::User::find_or_create_e2e_test_user(
        "殿馬一人",
        "kazuto.tonoma@e2e_test.com",
        db,
    )
    .await
    {
        Ok(u) => u,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let satonaka_user = match model::user::User::find_or_create_e2e_test_user(
        "里中智",
        "satoru.satonaka@e2e_test.com",
        db,
    )
    .await
    {
        Ok(u) => u,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    Ok(match login_user {
        "2" => tonoma_user,
        "3" => iwaki_user,
        "4" => satonaka_user,
        _ => yamada_user,
    })
}
