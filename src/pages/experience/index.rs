use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;
use std::sync::OnceLock;

use crate::{
    components::{self, Component, footer::footer, head::default_head, header::header, timeline},
    setup_language_loader,
};

use super::super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "experience")
}

pub const MOD_PATH: &str = module_path!();

pub const STYLE: &str = include_asset!("index.css");
pub const SCRIPT: &str = include_asset!("index.js");

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let core_loader = get_core_language_loader().select_languages(&[lang]);
    // let loader = get_language_loader().select_languages(&[lang]);
    let Component {
        html: timeline_html,
        style: timeline_style,
        ..
    } = timeline::component();

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                link rel="stylesheet" href=(page.path_to_root(lang)+*timeline_style);
                (default_head(&core_loader.get("Home"),"//TODO:", page, lang))
                style { (PreEscaped(STYLE)) }
            }

            body{
                (header(page, lang))
                main{
                    section.sect.content{(timeline_html(timeline::MarkupProps {
                        heading:"Erfahrung",
                        items: &[timeline::Item{content:r#"Ein Praktikum für das 5. Semester an der Thi.\ndarin habe ich..."#,content_long:"test",title:"ADVERMA GmbH",wide:false,year:"2025 - 2026"}],
                        start_left:true
                    }))}
                }
                (footer(lang))
                script{(PreEscaped(SCRIPT))}
            }
        },
    )
}
