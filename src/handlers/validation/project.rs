use crate::{handlers, model};
use anyhow::Result;
use firestore::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectValidation {
    pub project_info: Option<String>,
    pub project_name: Option<String>,
}

impl ProjectValidation {
    pub fn new() -> Self {
        Self {
            project_info: None,
            project_name: None,
        }
    }

    pub async fn validate_post(
        input: &handlers::project::ProjectInput,
        session: &model::session::Session,
        action: crate::Action,
        db: &FirestoreDb,
    ) -> Result<(
        Option<ProjectValidation>,
        Option<model::project::Project>,
        Option<model::project::ProjectMember>,
    )> {
        let mut validation = ProjectValidation::new();
        let mut project: Option<model::project::Project> = None;
        let mut member: Option<model::project::ProjectMember> = None;

        match action {
            // プロジェクト作成
            crate::Action::Create => {
                // TODO プロジェクト作成件数の制限を超えていたら作成できない。
                // TODO メンバーの数が制限を超えていたら作成できない。
            }

            // プロジェクト更新
            crate::Action::Update => {
                project = match model::project::Project::find(&input.project_id, &db).await {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e));
                    }
                };

                if let Some(p) = &project {
                    if p.deleted {
                        return Err(anyhow::anyhow!(
                            "プロジェクトが削除されています。".to_string()
                        ));
                    }
                } else {
                    return Err(anyhow::anyhow!("プロジェクトが存在しません。".to_string()));
                }

                // プロジェクトを更新できるのは、オーナーか管理者のみ
                member =
                    match model::project::ProjectMember::find(&input.project_id, &session.uid, &db)
                        .await
                    {
                        Ok(member) => member,
                        Err(e) => {
                            return Err(anyhow::anyhow!(e));
                        }
                    };

                let mut ok = false;
                if let Some(member) = &member {
                    if let Some(role) = member.role {
                        if role == model::project::ProjectRole::Owner as i32
                            || role == model::project::ProjectRole::Administrator as i32
                        {
                            ok = true;
                        }
                    }
                }
                if ok == false {
                    //let mut validation = Self::new();
                    validation.project_info =
                        Some("プロジェクト情報を更新する権限がありません".to_string());
                    return Ok((Some(validation), project, member));
                }

                // TODO メンバーの数が制限を超えていたら作成できない。

                // 読み込み時のタイムスタンプと現在のタイムスタンプを比較し、他のユーザーが更新していたら更新できない。
                let mut ok = false;
                if let Some(p) = &project {
                    if let Some(t) = p.updated_at {
                        if t.timestamp_micros().to_string() == input.timestamp {
                            ok = true;
                        }
                    }
                }
                if ok == false {
                    //let mut validation = Self::new();
                    validation.project_info =
                        Some("他のユーザーがプロジェクトを更新しため、更新できませんでした。<br>再読み込みを行ってください。".to_string());
                    return Ok((Some(validation), project, member));
                }
            }

            // プロジェクト削除
            crate::Action::Delete => {
                project = match model::project::Project::find(&input.project_id, &db).await {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e));
                    }
                };

                if let Some(p) = &project {
                    if p.deleted {
                        return Err(anyhow::anyhow!(
                            "プロジェクトが削除されています。".to_string()
                        ));
                    }
                } else {
                    return Err(anyhow::anyhow!("プロジェクトが存在しません。".to_string()));
                }

                // プロジェクトを削除できるのはオーナーのみ
                member =
                    match model::project::ProjectMember::find(&input.project_id, &session.uid, &db)
                        .await
                    {
                        Ok(member) => member,
                        Err(e) => {
                            return Err(anyhow::anyhow!(e));
                        }
                    };

                let mut ok = false;
                if let Some(member) = &member {
                    if let Some(role) = member.role {
                        if role == model::project::ProjectRole::Owner as i32 {
                            ok = true;
                        }
                    }
                }
                if ok == false {
                    //let mut validation = Self::new();
                    validation.project_info =
                        Some("プロジェクトを削除する権限がありません".to_string());
                    return Ok((Some(validation), None, member));
                }
            }

            _ => {}
        }

        match action {
            crate::Action::Create | crate::Action::Update => {
                let project_name = input.project_name.trim().to_string();
                if project_name.len() == 0 {
                    //let mut validation = Self::new();
                    validation.project_name = Some("入力してください".to_string());
                    return Ok((Some(validation), project, member));
                }

                let projects = match model::project::Project::find_by_owner_and_name(
                    &session.uid,
                    &project_name,
                    &db,
                )
                .await
                {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e));
                    }
                };

                for prj in projects {
                    if prj.deleted || prj.id == input.project_id {
                        continue;
                    }

                    //let mut validation = Self::new();
                    validation.project_name =
                        Some("同じ名前のプロジェクトが存在します".to_string());
                    return Ok((Some(validation), project, member));
                }
            }
            _ => {}
        }

        Ok((None, project, member))
    }
}
