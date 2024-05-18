use super::super::Component;
use super::parts::{
    footer::Footer, nav::Nav, project_history::ProjectHistory, project_info::ProjectInfo,
    project_note::ProjectNote,
};
use crate::ProjectTab;
use crate::Props;

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
                        if let None = &props.project {
                            *buf += r#"を作成"#;
                        }
                        *buf += r#"</h3>"#;
                        if let Some(p) = &props.project {
                            *buf += r#"<div class="pb-2">"#;
                            {
                                *buf += r#"<ul class="nav nav-tabs">"#;
                                {
                                    *buf += r#"<li class="nav-item">"#;
                                    {
                                        *buf += r#"<a class="nav-link"#;
                                        if props.project_tab == ProjectTab::Info {
                                            *buf += r#" active"#;
                                        }
                                        *buf += r#"" href="/project/info?id="#;
                                        *buf += &p.id.clone().unwrap();
                                        *buf += r#"&tab=info">基本情報</a>"#;
                                    }
                                    *buf += r#"</li>"#;
                                    *buf += r#"<li class="nav-item">"#;
                                    {
                                        *buf += r#"<a class="nav-link"#;
                                        if props.project_tab == ProjectTab::Note {
                                            *buf += r#" active"#;
                                        }
                                        *buf += r#"" href="/project/info?id="#;
                                        *buf += &p.id.clone().unwrap();
                                        *buf += r#"&tab=note">ノート</a>"#;
                                    }
                                    *buf += r#"</li>"#;
                                    *buf += r#"<li class="nav-item">"#;
                                    {
                                        *buf += r#"<a class="nav-link"#;
                                        if props.project_tab == ProjectTab::History {
                                            *buf += r#" active"#;
                                        }
                                        *buf += r#"" href="/project/info?id="#;
                                        *buf += &p.id.clone().unwrap();
                                        *buf += r#"&tab=history">更新履歴</a>"#;
                                    }
                                    *buf += r#"</li>"#;
                                }
                                *buf += r#"</ul>"#;
                            }
                            *buf += r#"</div>"#;
                        }

                        match &props.project_tab {
                            ProjectTab::Info => {
                                self.project_info.write(props, buf);
                            }
                            ProjectTab::Note => {
                                self.project_note.write(props, buf);
                            }
                            ProjectTab::History => {
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
            match &props.project_tab {
                ProjectTab::Info => {
                    *buf += r#"<script src="/static/js/project0012.js"></script>"#;
                }
                ProjectTab::Note => {
                    *buf += r#"<script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>"#;
                    *buf += r#"<script src="/static/js/markdown0012.js"></script>"#;
                }
                _ => {}
            }
        }
        *buf += r#"</body>"#;
    }
}
