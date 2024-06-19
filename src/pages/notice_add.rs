use crate::{components::body::notice_add::NoticeAddBody, handlers::validation, pages::page};

/// Component for rendering the homepage
pub struct NoticeAddPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl NoticeAddPage {
    pub fn new(
        props: page::Props,
        message: Option<String>,
        password: Option<String>,
        validation: Option<validation::notice::NoticeValidation>,
        notice_id: Option<String>,
    ) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = NoticeAddBody::new(message, password, validation, notice_id);
        page.body = Some(Box::new(body));

        NoticeAddPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
