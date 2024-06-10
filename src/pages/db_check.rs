use crate::{components::body::db_check::DbCheckBody, handlers::validation, pages::page};

/// Component for rendering the homepage
pub struct DbCheckPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl DbCheckPage {
    pub fn new(
        props: page::Props,
        password: Option<String>,
        validation: Option<validation::db_check::DbCheckValidation>,
    ) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = DbCheckBody::new(password, validation);
        page.body = Some(Box::new(body));

        DbCheckPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
