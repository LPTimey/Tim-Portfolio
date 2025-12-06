use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;
use std::{sync::OnceLock, time::Duration};

use crate::{
    components::{
        self, Component, footer::footer, head::default_head, header::header, icon::Icon,
        project_card, project_table::Content, scrolling_img, tooltip,
    },
    include_public, setup_language_loader,
};

use super::super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "adverma")
}

pub const MOD_PATH: &str = module_path!();

pub const STYLE: &str = include_asset!("index.css");
pub const SCRIPT: &str = include_asset!("index.js");

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let core_loader = get_core_language_loader().select_languages(&[lang]);
    // let loader = get_language_loader().select_languages(&[lang]);


    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head(&core_loader.get("Home"),"//TODO:", page, lang))
                style { (PreEscaped(STYLE)) }
            }

            body{
                (header(page, lang))
                main{
                }
                (footer(lang))
                script{(PreEscaped(SCRIPT))}
            }
        },
    )
}
