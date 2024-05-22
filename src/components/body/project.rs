use super::super::Component;
use super::parts::{
    footer::Footer, nav::Nav, project_history::ProjectHistory, project_info::ProjectInfo,
    project_note::ProjectNote,
};
use crate::Props;
use crate::Tab;

pub struct ProjectBody {
    pub nav: Box<dyn Component + Send>,
    pub project_info: Box<dyn Component + Send>,
    pub project_note: Box<dyn Component + Send>,
    pub project_history: Box<dyn Component + Send>,
    pub footer: Box<dyn Component + Send>,
}

impl ProjectBody {
    pub fn new() -> Self {
        ProjectBody {
            nav: Box::new(Nav {}),
            project_info: Box::new(ProjectInfo {}),
            project_note: Box::new(ProjectNote {}),
            project_history: Box::new(ProjectHistory {}),
            footer: Box::new(Footer {}),
        }
    }
}

impl Component for ProjectBody {
    fn write(&self, props: &Props, buf: &mut String) {
        *buf += r#"<body>"#;
        {
            self.nav.write(props, buf);

            *buf += r#"<main>"#;
            {
                *buf += r#"<div class="py-3">"#;
                {
                    *buf += r#"<div class="container">"#;
                    {
                        *buf += r#"<h3 class="mb-3">プロジェクト"#;
                        if props.is_create {
                            *buf += r#"を作成"#;
                        }
                        *buf += r#"</h3>"#;

                        if let Some(v) = &props.project_validation {
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

                        if props.is_create == false {
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
                                            *buf += &p.id.clone().unwrap();
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
                                            *buf += &p.id.clone().unwrap();
                                            *buf += r#"&tab=note">ノート</a>"#;
                                        }
                                        *buf += r#"</li>"#;

                                        *buf += r#"<li class="nav-item">"#;
                                        {
                                            *buf += r#"<a class="nav-link"#;
                                            if props.tab == Tab::History {
                                                *buf += r#" active"#;
                                            }
                                            *buf += r#"" href="/project?id="#;
                                            *buf += &p.id.clone().unwrap();
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
                            Tab::History => {
                                self.project_history.write(props, buf);
                            }
                        }
                    }
                    *buf += r#"</div>"#;
                }
                *buf += r#"</div>"#;
            }
            *buf += r#"</main>"#;

            self.footer.write(props, buf);
            *buf += r#"<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>"#;
            match &props.tab {
                Tab::Info => {
                    *buf += r#"<script src="/static/js/project0012b.js"></script>"#;
                }
                Tab::Note => {
                    *buf += r#"<script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>"#;
                    *buf += r#"<script src="/static/js/markdown0012.js"></script>"#;
                }
                _ => {}
            }
        }
        *buf += r#"</body>"#;
    }
}
