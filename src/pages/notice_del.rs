use crate::{components::body::notice_del::NoticeDelBody, handlers::validation, pages::page};

/// Component for rendering the homepage
pub struct NoticeDelPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl NoticeDelPage {
    pub fn new(
        props: page::Props,
        notice_id: Option<String>,
        password: Option<String>,
        validation: Option<validation::notice::NoticeValidation>,
    ) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = NoticeDelBody::new(notice_id, password, validation);
        page.body = Some(Box::new(body));

        NoticeDelPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
