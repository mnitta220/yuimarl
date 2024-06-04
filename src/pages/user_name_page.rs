use crate::{components::body::user_name::UserNameBody, pages::page};

/// Component for rendering the contact page
pub struct UserNamePage {
    pub props: page::Props,
    pub page: page::Page,
}

impl UserNamePage {
    pub fn new(props: page::Props) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = UserNameBody::new();
        page.body = Some(Box::new(body));

        UserNamePage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
