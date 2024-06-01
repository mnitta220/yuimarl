use crate::{components::body::ticket_list::TicketListBody, pages::page};

/// Component for rendering the Ticket List page.
pub struct TicketListPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl TicketListPage {
    pub fn new(props: page::Props) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = TicketListBody::new();
        page.body = Some(Box::new(body));

        TicketListPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
