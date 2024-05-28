use crate::{components::body::db_check::DbCheckBody, pages::page};

/// Component for rendering the homepage
pub struct DbCheckPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl DbCheckPage {
    pub fn new(props: page::Props) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = DbCheckBody::new();
        page.body = Some(Box::new(body));

        DbCheckPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
