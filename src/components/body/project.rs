use super::super::Component;
use super::parts::{
    footer::Footer, gantt_chart::GanttChart, loading, nav::Nav, project_history::ProjectHistory,
    project_info::ProjectInfo, project_note::ProjectNote,
};
use crate::{handlers::validation, Props, Tab};

pub struct ProjectBody {
    pub nav: Box<dyn Component + Send>,
    pub project_info: Box<dyn Component + Send>,
    pub project_note: Box<dyn Component + Send>,
    pub gantt_chart: Box<dyn Component + Send>,
    pub project_history: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
    pub validation: Option<validation::project::ProjectValidation>,
}

impl ProjectBody {
    pub fn new(
        can_update: bool,
        can_delete: bool,
        validation: Option<validation::project::ProjectValidation>,
    ) -> Self {
        ProjectBody {
            nav: Box::new(Nav {}),
            project_info: Box::new(ProjectInfo {
                can_update,
                can_delete,
                validation: validation.clone(),
            }),
            project_note: Box::new(ProjectNote { can_update }),
            gantt_chart: Box::new(GanttChart { can_update }),
            project_history: Box::new(ProjectHistory {}),
            footer: Box::new(Footer {}),
            validation,
        }
    }
}

impl Component for ProjectBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            // ローディング画面
            if props.tab == Tab::GanttChart {
                loading::loading(buf);
            }

            self.nav.write(props, buf);

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="py-3">"#;
                {
                    *buf += r#"<div class="container">"#;
                    {
                        *buf += r#"<h3 class="mb-3">プロジェクト"#;
                        if props.action == crate::Action::Create {
                            *buf += r#"を作成"#;
                        }
                        *buf += r#"</h3>"#;

                        if let Some(v) = &self.validation {
                            if let Some(e) = &v.project_info {
                                *buf += r#"<div class="row p-2">"#;
                                {
                                    *buf += r#"<div class="alert alert-danger text-start" role="alert">"#;
                                    *buf += e;
                                    *buf += r#"</div>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                        }

                        if props.action != crate::Action::Create {
                            if let Some(p) = &props.project {
                                *buf += r#"<div class="pb-2">"#;
                                {
                                    *buf += r#"<ul class="nav nav-tabs">"#;
                                    {
                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::Info {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/project?id="#;
                                            *buf += &p.id;
                                            *buf += r#"&tab=info">基本情報</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::Note {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/project?id="#;
                                            *buf += &p.id;
                                            *buf += r#"&tab=note">ノート</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::GanttChart {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/project?id="#;
                                            *buf += &p.id;
                                            *buf += r#"&tab=gantt">ガントチャート</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::History {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/project?id="#;
                                            *buf += &p.id;
                                            *buf += r#"&tab=history">更新履歴</a>"#;
                                        }
                                        *buf += r#"</li>"#;
                                    }
                                    *buf += r#"</ul>"#;
                                }
                                *buf += r#"</div>"#;
                            }
                        }

                        match &props.tab {
                            Tab::Info => {
                                self.project_info.write(props, buf);
                            }
                            Tab::Note => {
                                self.project_note.write(props, buf);
                            }
                            Tab::GanttChart => {
                                self.gantt_chart.write(props, buf);
                            }
                            Tab::History => {
                                self.project_history.write(props, buf);
                            }
                            _ => {}
                        }
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
            match &props.tab {
                Tab::Info => {
                    *buf +=
                        r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
                    *buf += r#"<script src="/static/js/project1015.js"></script>"#;
                }
                Tab::Note => {
                    *buf +=
                        r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
                    *buf += r#"<script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>"#;
                    *buf += r#"<script src="/static/js/markdown1015.js"></script>"#;
                }
                _ => {}
            }
        }
        *buf += r#"</body>"#;
    }
}
