use crate::{components::body::e2e_test::E2eTestBody, handlers::validation, pages::page};

/// Component for rendering the homepage
pub struct E2eTestPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl E2eTestPage {
    pub fn new(
        props: page::Props,
        password: Option<String>,
        validation: Option<validation::e2e_test::E2eTestValidation>,
    ) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = E2eTestBody::new(password, validation);
        page.body = Some(Box::new(body));

        E2eTestPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
