use crate::{components::body::agreement::AgreementBody, pages::page};

/// Component for rendering the contact page
pub struct AgreementPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl AgreementPage {
    pub fn new(props: page::Props, disagree: bool) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = AgreementBody::new(disagree);
        page.body = Some(Box::new(body));

        AgreementPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
