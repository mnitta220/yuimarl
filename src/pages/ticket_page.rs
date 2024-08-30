use crate::{components::body::ticket::TicketBody, handlers::validation, pages::page};

/// Component for rendering the contact page
pub struct TicketPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl TicketPage {
    pub fn new(
        props: page::Props,
        can_update: bool,
        can_delete: bool,
        validation: Option<validation::ticket::TicketValidation>,
    ) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = TicketBody::new(can_update, can_delete, validation);
        page.body = Some(Box::new(body));

        TicketPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
