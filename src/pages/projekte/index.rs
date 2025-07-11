use crate::components::{
    Component, footer::footer, head::default_head, header::header, project_card,
};

use super::super::*;

pub const MOD_PATH: &str = module_path!();

pub fn page(page: Page) -> maud::Markup {
    let Component {
        html: project_card_html,
        style: card_style,
        ..
    } = project_card::component();

    html! {
        head{
            (default_head("Projekte","TODO: Add description",page.path_to_root()))
            link rel="stylesheet" href=(page.path_to_root() + *card_style );
        }

        body{
            (header(page))
            main{
                @for project in Page::projects(){
                    (project_card_html(
                        project_card::MarkupProps {
                            data: project,
                            path_to_root: page.path_to_root(),
                            is_in_grid: false,
                            reactive_color: false,
                        }
                    ))
                }
            }
            (footer())
        }
    }
}
