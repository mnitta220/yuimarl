use crate::{components::body::contact::ContactBody, pages::page};

/// Component for rendering the contact page
pub struct ContactPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl ContactPage {
    pub fn new(props: page::Props) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = ContactBody::new();
        page.body = Some(Box::new(body));

        ContactPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
