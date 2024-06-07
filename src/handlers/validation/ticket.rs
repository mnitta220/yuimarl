use crate::{handlers, model};
use anyhow::Result;
use firestore::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketValidation {
    pub info: Option<String>,
    pub name: Option<String>,
}

impl TicketValidation {
    pub fn new() -> Self {
        Self {
            info: None,
            name: None,
        }
    }

    pub async fn validate_post(
        input: &handlers::ticket::TicketInput,
        session: &model::session::Session,
        action: crate::Action,
        db: &FirestoreDb,
    ) -> Result<(
        Option<TicketValidation>,
        Option<model::project::Project>,
        Option<model::project::ProjectMember>,
        Option<model::ticket::Ticket>,
    )> {
        let mut validation = TicketValidation::new();
        let mut ticket: Option<model::ticket::Ticket> = None;

        let project = match model::project::Project::find(&input.project_id, &db).await {
            Ok(p) => p,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

        //let mut ok = false;
        if let Some(p) = &project {
            if p.deleted {
                return Err(anyhow::anyhow!(
                    "プロジェクトが削除されています。".to_string()
                ));
            }
        } else {
            return Err(anyhow::anyhow!("プロジェクトが存在しません。".to_string()));
        }

        let project_member =
            match model::project::ProjectMember::find(&input.project_id, &session.uid, &db).await {
                Ok(p) => p,
                Err(e) => {
                    return Err(anyhow::anyhow!(e));
                }
            };

        if project_member.is_none() {
            return Err(anyhow::anyhow!(
                "プロジェクトのメンバーではありません。".to_string()
            ));
        }

        match action {
            // チケット作成
            crate::Action::Create => {
                // TODO チケット作成権限がなければ作成できない。
                // TODO チケット作成件数の制限を超えていたら作成できない。
                // TODO チケット名が入力されていなければ作成できない。
                // TODO チケット名が重複していたら作成できない。
            }

            // チケット更新
            crate::Action::Update => {
                ticket = match model::ticket::Ticket::find(&input.ticket_id, &db).await {
                    Ok(t) => t,
                    Err(e) => {
                        return Err(anyhow::anyhow!(e));
                    }
                };
                if let Some(t) = &ticket {
                    // 読み込み時のタイムスタンプと現在のタイムスタンプを比較し、他のユーザーが更新していたら更新できない。
                    let mut ok = false;
                    if let Some(ts) = &t.updated_at {
                        if ts.timestamp_micros().to_string() == input.timestamp {
                            ok = true;
                        }
                    }
                    if !ok {
                        validation.info = Some("他のユーザーがチケットを更新しため、更新できませんでした。<br>再読み込みを行ってください。".to_string());
                        return Ok((Some(validation), project, project_member, ticket));
                    }
                } else {
                    return Err(anyhow::anyhow!("チケットが存在しません。".to_string()));
                }
            }

            // チケット削除
            crate::Action::Delete => {}
            _ => {}
        }

        match action {
            crate::Action::Create | crate::Action::Update => {
                let name = input.name.trim().to_string();
                if name.len() == 0 {
                    //let mut validation = Self::new();
                    validation.name = Some("入力してください".to_string());
                    return Ok((Some(validation), project, project_member, ticket));
                }
            }
            _ => {}
        }

        Ok((None, project, project_member, ticket))
    }
}
