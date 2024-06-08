use crate::{components::body::project_list::ProjectListBody, pages::page};

/// Component for rendering the homepage
pub struct ProjectListPage {
    pub props: page::Props,
    pub page: page::Page,
}

impl ProjectListPage {
    pub fn new(props: page::Props, owner_cnt: i32) -> Self {
        // Construct the components of the HTML page.
        let mut page = page::Page::new();

        // Construct the components of the HTML body.
        let body = ProjectListBody::new(owner_cnt);
        page.body = Some(Box::new(body));

        ProjectListPage { props, page }
    }

    pub fn write(&mut self) -> String {
        self.page.write(&self.props)
    }
}
