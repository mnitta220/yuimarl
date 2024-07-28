use super::ticket::Ticket;
use crate::model;
use anyhow::Result;
use chrono::{DateTime, Utc};
use firestore::*;
use futures::stream::BoxStream;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub const COLLECTION_NAME: &'static str = "ticket";

/// ガントチャート用チケット
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GanttTicket {
    pub id: String,                        // ID(uuid)
    pub id_disp: Option<String>,           // 表示用チケットID（接頭辞＋連番）
    pub name: Option<String>,              // チケット名
    pub start_date: Option<String>,        // 開始日
    pub end_date: Option<String>,          // 終了日
    pub progress: i32,                     // 進捗率
    pub parent_id: Option<String>,         // 親チケットID
    pub ganttseq: Option<i32>,             // ガントチャート表示順
    pub updated_at: Option<DateTime<Utc>>, // 更新日時
    pub children: Vec<GanttTicket>,        // 子チケット
    pub open: bool,                        //
}

impl GanttTicket {
    pub fn new(ticket: &Ticket) -> Self {
        Self {
            id: ticket.id.clone(),
            id_disp: ticket.id_disp.clone(),
            name: ticket.name.clone(),
            start_date: ticket.start_date.clone(),
            end_date: ticket.end_date.clone(),
            progress: ticket.progress,
            parent_id: ticket.parent_id.clone(),
            ganttseq: ticket.ganttseq,
            updated_at: ticket.updated_at,
            children: Vec::new(),
            open: false,
        }
    }

    /// ガントチャートに表示するチケットを取得する
    pub async fn load_gantt(
        project_id: &str,
        db: &FirestoreDb,
    ) -> Result<(Vec<GanttTicket>, String, String)> {
        let object_stream: BoxStream<FirestoreResult<Ticket>> = match db
            .fluent()
            .select()
            .fields(paths!(Ticket::{id, project_id, id_disp, name, progress, priority, start_date, end_date, parent_id, ganttseq, updated_at}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Ticket::project_id)).eq(&project_id),
                    q.field(path!(Ticket::ganttchart)).eq(&true),
                ])
            })
            .order_by(vec![
                (path!(Ticket::id), FirestoreQueryDirection::Ascending),
            ])
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let tickets: Vec<Ticket> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut gantts: Vec<GanttTicket> = Vec::new();

        // 親チケットがないチケットを最上位に表示する
        for ticket in &tickets {
            if ticket.parent_id.is_none() {
                let gantt = GanttTicket::new(ticket);
                gantts.push(gantt);
            }
        }

        let (mut gantts, min, max) = GanttTicket::gantt_sub(gantts, &tickets);

        GanttTicket::gantt_sort(&mut gantts);

        Ok((gantts, min, max))
    }

    /// 子チケットを親チケットのchildrenに設定する
    fn gantt_sub(
        gantts: Vec<GanttTicket>,
        tickets: &Vec<Ticket>,
    ) -> (Vec<GanttTicket>, String, String) {
        let mut newone: Vec<GanttTicket> = Vec::new();
        let mut min = String::from("");
        let mut max = String::from("");

        for mut gantt in gantts {
            let iter = tickets.iter().filter(|x| match &x.parent_id {
                Some(pid) => pid == &gantt.id,
                None => false,
            });

            for it in iter {
                let child = GanttTicket::new(it);
                gantt.children.push(child);
            }

            if let Some(s) = &gantt.start_date {
                if min.len() == 0 || min > *s {
                    min = s.clone();
                }
            }
            if let Some(e) = &gantt.end_date {
                if max.len() == 0 || max < *e {
                    max = e.clone();
                }
            }

            let (ts, mi, ma) = GanttTicket::gantt_sub(gantt.children, tickets);

            gantt.children = ts;
            if mi.len() > 0 && (min.len() == 0 || min > mi) {
                min = mi;
            }
            if ma.len() > 0 && (max.len() == 0 || max < ma) {
                max = ma;
            }

            newone.push(gantt);
        }

        (newone, min, max)
    }

    fn gantt_sort(gantts: &mut Vec<GanttTicket>) {
        gantts.sort_by(|a, b| {
            if let Some(sa) = &a.ganttseq {
                if let Some(sb) = &b.ganttseq {
                    if sa < sb {
                        return Ordering::Less;
                    } else if sa > sb {
                        return Ordering::Greater;
                    }
                } else {
                    return Ordering::Less;
                }
            } else {
                if b.ganttseq.is_some() {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        });

        for gantt in gantts {
            GanttTicket::gantt_sort(&mut gantt.children);
        }
    }

    /// ガントチャートを更新する
    pub async fn update_gantt(
        session: &model::session::Session,
        project_id: &str,
        tickets_upd: &Vec<GanttTicket>,
        db: &FirestoreDb,
    ) -> Result<()> {
        let updated = "他のユーザーがチケットを更新しため、更新できませんでした。<br>再読み込みを行ってください。".to_string();

        let member = match model::project::ProjectMember::find(&project_id, &session.uid, &db).await
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
            return Err(anyhow::anyhow!(
                "プロジェクト情報を更新する権限がありません".to_string()
            ));
        }

        // 現在のデータ
        let object_stream: BoxStream<FirestoreResult<Ticket>> = match db
            .fluent()
            .select()
            .fields(paths!(Ticket::{id, project_id, id_disp, name, progress, priority, start_date, end_date, parent_id, ganttseq, updated_at}))
            .from(COLLECTION_NAME)
            .filter(|q| {
                q.for_all([
                    q.field(path!(Ticket::project_id)).eq(&project_id),
                    q.field(path!(Ticket::ganttchart)).eq(&true),
                ])
            })
            .order_by(vec![
                (path!(Ticket::id), FirestoreQueryDirection::Ascending),
            ])
            .obj()
            .stream_query_with_errors()
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let tickets_cur: Vec<Ticket> = match object_stream.try_collect().await {
            Ok(s) => s,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        let mut gantts: Vec<GanttTicket> = Vec::new();

        // 親チケットがないチケットを最上位に表示する
        for ticket in &tickets_cur {
            if ticket.parent_id.is_none() {
                let gantt = GanttTicket::new(ticket);
                gantts.push(gantt);
            }
        }

        let (mut gantts, _min, _max) = GanttTicket::gantt_sub(gantts, &tickets_cur);
        GanttTicket::gantt_sort(&mut gantts);
        let tickets_cur = GanttTicket::flatten(&gantts, false);

        let tickets_upd = GanttTicket::flatten(tickets_upd, true);

        /*
         * チケットの更新処理
         * 現在のチケットと更新後のチケットをマッチングして、更新処理を行う。
         */
        let mut c = 0;
        let mut current = tickets_cur.get(c);
        let mut u = 0;
        let mut upd = tickets_upd.get(u);
        // トランザクション開始
        let mut transaction = match db.begin_transaction().await {
            Ok(t) => t,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()));
            }
        };

        loop {
            if let Some(up) = upd {
                if let Some(cur) = current {
                    match up.id.cmp(&cur.id) {
                        Ordering::Less => {
                            return Err(anyhow::anyhow!(updated));
                        }

                        Ordering::Greater => {
                            return Err(anyhow::anyhow!(updated));
                        }

                        Ordering::Equal => {
                            let mut ne = false;

                            if let Some(curp) = &cur.parent_id {
                                if let Some(upp) = &up.parent_id {
                                    if curp != upp {
                                        ne = true;
                                    }
                                } else {
                                    ne = true;
                                }
                            } else {
                                if up.parent_id.is_some() {
                                    ne = true;
                                }
                            }

                            if let Some(curs) = cur.ganttseq {
                                if let Some(ups) = up.ganttseq {
                                    if curs != ups {
                                        ne = true;
                                    }
                                } else {
                                    ne = true;
                                }
                            } else {
                                if up.ganttseq.is_some() {
                                    ne = true;
                                }
                            }

                            if ne {
                                if let Some(curu) = cur.updated_at {
                                    if let Some(uppu) = up.updated_at {
                                        if curu.timestamp_micros() != uppu.timestamp_micros() {
                                            return Err(anyhow::anyhow!(updated));
                                        }
                                    }
                                }

                                let mut cur = cur.clone();
                                cur.parent_id = up.parent_id.clone();
                                cur.ganttseq = up.ganttseq;
                                cur.updated_at = Some(Utc::now());

                                if let Err(e) = db
                                    .fluent()
                                    .update()
                                    .fields(paths!(
                                        Ticket::parent_id,
                                        Ticket::ganttseq,
                                        Ticket::updated_at
                                    ))
                                    .in_col(&COLLECTION_NAME)
                                    .document_id(&cur.id)
                                    .object(&cur)
                                    .add_to_transaction(&mut transaction)
                                {
                                    return Err(anyhow::anyhow!(e.to_string()));
                                }
                            }

                            c += 1;
                            current = tickets_cur.get(c);
                            u += 1;
                            upd = tickets_upd.get(u);
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!(updated));
                }
            } else {
                break;
            }
        }

        // トランザクションコミット
        if let Err(e) = transaction.commit().await {
            return Err(anyhow::anyhow!(e.to_string()));
        }

        Ok(())
    }

    /// 更新するデータを直列化する
    fn flatten(tickets_upd: &Vec<GanttTicket>, set_seq: bool) -> Vec<GanttTicket> {
        let mut tickets: Vec<GanttTicket> = Vec::new();

        let mut i = 0;
        for t in tickets_upd {
            let mut ticket = t.clone();
            if set_seq {
                ticket.ganttseq = Some(i);
            }
            ticket.parent_id = None;
            ticket.children.clear();
            tickets.push(ticket);
            GanttTicket::flatten_sub(Some(t.id.clone()), &t.children, set_seq, &mut tickets);
            i += 1;
        }

        tickets.sort_by(|a, b| a.id.cmp(&b.id));

        tickets
    }

    fn flatten_sub(
        parent: Option<String>,
        tickets_upd: &Vec<GanttTicket>,
        set_seq: bool,
        tickets: &mut Vec<GanttTicket>,
    ) {
        let mut i = 0;
        for t in tickets_upd {
            let mut ticket = t.clone();
            ticket.ganttseq = Some(i);
            ticket.parent_id = parent.clone();
            ticket.children.clear();
            tickets.push(ticket);
            GanttTicket::flatten_sub(Some(t.id.clone()), &t.children, set_seq, tickets);
            i += 1;
        }
    }
}
