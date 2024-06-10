use crate::{components::body::project::ProjectBody, handlers::validation, pages::page};

/// Component for rendering the homepage
pub struct ProjectPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl ProjectPage {
    pub fn new(
        props: page::Props,
        can_update: bool,
        can_delete: bool,
        validation: Option<validation::project::ProjectValidation>,
    ) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = ProjectBody::new(can_update, can_delete, validation);
        page.body = Some(Box::new(body));

        ProjectPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
