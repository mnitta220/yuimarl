use super::session::Session;
use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const COLLECTION_NAME: &'static str = "ticket";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket {
    pub id: String,
    pub project_id: String,
    pub id_disp: String,
    pub name: String,
    pub note: String,
    pub start_date: String,
    pub end_date: String,
    pub progress: i32,
    pub priority: String,
    pub parent: String,
    pub owner: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketValidation {
    pub name: Option<String>,
}

impl Ticket {
    pub async fn insert(
        input: &crate::handlers::ticket::TicketCreateInput,
        session: &Session,
        project: &super::project::Project,
        db: &FirestoreDb,
    ) -> Result<()> {
        let ticket_number = project.ticket_number + 1;
        let id_disp = format!("{}{}", project.prefix, ticket_number);
        let progress = match input.progress.parse::<i32>() {
            Ok(p) => p,
            Err(_) => 0,
        };
        let mut ticket = Ticket {
            id: "".to_string(),
            project_id: project.id.clone(),
            id_disp: id_disp,
            name: input.name.clone(),
            note: input.note.clone(),
            start_date: input.start_date.clone(),
            end_date: input.end_date.clone(),
            progress: progress,
            priority: input.priority.clone(),
            parent: "".to_string(),
            owner: session.uid.clone(),
            created_at: Utc::now(),
        };
        let mut count = 0u32;

        loop {
            count += 1;
            if count > 9 {
                return Err(anyhow::anyhow!("Failed to create ticket".to_string()));
            }
            let id = Uuid::now_v7().to_string();
            ticket.id = id.clone();

            match db
                .fluent()
                .insert()
                .into(&COLLECTION_NAME)
                .document_id(id)
                .object(&ticket)
                .execute::<Ticket>()
                .await
            {
                Ok(_) => {
                    break;
                }
                Err(e) => match &e {
                    firestore::errors::FirestoreError::DataConflictError(e) => {
                        tracing::error!("DataConflictError: {:?}", e);
                        continue;
                    }
                    _ => {
                        return Err(anyhow::anyhow!(e.to_string()));
                    }
                },
            };
        }

        let project_new = super::project::Project {
            ticket_number: ticket_number,
            ..project.clone()
        };
        if let Err(e) = db
            .fluent()
            .update()
            .fields(paths!(super::project::Project::ticket_number))
            .in_col(&super::project::COLLECTION_NAME)
            .document_id(&project.id)
            .object(&project_new)
            .execute::<super::project::Project>()
            .await
        {
            return Err(anyhow::anyhow!(e.to_string()));
        }
        tracing::debug!("Ticket inserted {:?}", ticket);

        Ok(())
    }
}
