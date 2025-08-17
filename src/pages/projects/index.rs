use maud::PreEscaped;

use crate::{
    components::{
        Component, footer::footer, head::default_head, header::header, page, project_card,
    },
    projects::SortProjects,
};

use super::super::*;

pub const MOD_PATH: &str = module_path!();

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let Component {
        html: project_card_html,
        style: card_style,
        ..
    } = project_card::component();

    page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head("Projekte","TODO: Add description",page,lang))
                style{
                    (PreEscaped(include_asset!("projekte.css")))
                }
                link rel="stylesheet" href=(page.path_to_root(lang) + *card_style );
            }

            body{
                (header(page,lang))
                div."dodge-header"{}
                main{

                    section #Projects .content{
                        @for project in Page::projects(lang).sort_by_name(){
                        (project_card_html(
                            project_card::MarkupProps {
                                data: project,
                                path_to_root: page.path_to_root(lang),
                                is_in_grid: true,
                                reactive_color: true,
                                lang
                            }
                        ))
                        }
                    }
                }
                (footer(lang))
            }
        },
    )
}
