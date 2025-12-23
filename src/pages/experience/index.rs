use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;
use std::sync::OnceLock;

use crate::{
    components::{
        self, Component,
        footer::footer,
        head::default_head,
        header::header,
        timeline,
    },
    lang_to_html, setup_language_loader,
};
pub const ICH: Link = link_public!("assets/Lebenslauf/schönes bild klein bg@0,33x.jpg");

use super::super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "experience")
}

pub const MOD_PATH: &str = module_path!();

pub const STYLE: &str = include_asset!("erfahrung.css");

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let core_loader = get_core_language_loader().select_languages(&[lang]);
    let loader = get_language_loader().select_languages(&[lang]);
    let Component {
        html: timeline_html,
        style: timeline_style,
        script: timeline_script,
    } = timeline::component();

    let items = [
        timeline::Item {
            content: PreEscaped(lang_to_html(&loader.get("adverma-short"))),
            content_long: html! {
                div style="display:flex; justify-content: space-between;"{
                    p {(lang_to_html(&loader.get("adverma-long")))}
                    picture style="width:fit-content"{img style="height:10rem;" draggable="false" src=(page.path_to_root(lang) + *ICH) alt="";}
                }
            },
            title: "ADVERMA GmbH",
            wide: false,
            year: "2025 - 2026",
        },
        timeline::Item {
            content: PreEscaped(lang_to_html(&loader.get("sem-4-short"))),
            content_long: PreEscaped(lang_to_html(&loader.get("sem-4-long"))),
            title: "THI - Semester 4",
            wide: true,
            year: "2025",
        },
        timeline::Item {
            content: PreEscaped(lang_to_html(&loader.get("sem-3-short"))),
            content_long: PreEscaped(lang_to_html(&loader.get("sem-3-long"))),
            title: "THI - Semester 3",
            wide: false,
            year: "2024 - 2025",
        },
        timeline::Item {
            content: PreEscaped(lang_to_html(&loader.get("sem-2-short"))),
            content_long: PreEscaped(lang_to_html(&loader.get("sem-2-long"))),
            title: "THI - Semester 2",
            wide: true,
            year: "2024",
        },
        timeline::Item {
            content: PreEscaped(lang_to_html(&loader.get("sem-1-short"))),
            content_long: PreEscaped(lang_to_html(&loader.get("sem-1-long"))),
            title: "THI - Semester 1",
            wide: false,
            year: "2023 - 2024",
        },
    ];

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                script type="module" src=(page.path_to_root(lang)+*timeline_script){}
                link rel="stylesheet" href=(page.path_to_root(lang)+*timeline_style);
                (default_head(&core_loader.get("Experience"),"//TODO:", page, lang))
            }

            body{
                style { (PreEscaped(STYLE)) }
                (header(page, lang))
                main{
                    section.sect.content{(timeline_html(timeline::MarkupProps {
                        heading:"Erfahrung",
                        level: 1,
                        items: items,
                        start_left:true
                    }))}
                }
                (footer(lang))
                // script{(PreEscaped(SCRIPT))}
            }
        },
    )
}
