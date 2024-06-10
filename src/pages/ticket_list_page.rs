use super::super::handlers::ticket_list::{TicketListInput, TicketListProps};
use crate::{components::body::ticket_list::TicketListBody, pages::page};

pub const PAGE_COUNT: usize = 25;

/// Component for rendering the Ticket List page.
pub struct TicketListPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl TicketListPage {
    pub fn new(props: page::Props, input: TicketListInput, list_props: TicketListProps) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = TicketListBody::new(input, list_props);
        page.body = Some(Box::new(body));

        TicketListPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
