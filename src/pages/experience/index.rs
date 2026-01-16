use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;
use std::sync::OnceLock;

use crate::{
    assets::img::{Img, ImgProps}, components::{
        self, Component, badge, footer::footer, head::default_head, header::header, timeline,
    }, lang_to_html, setup_language_loader
};

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
    let Component {
        style: badge_style, ..
    } = badge::component();
    let profile_img = Img::new("public", "assets/Lebenslauf/schönes bild.JPG", "").unwrap();

    let items: [(timeline::Item<'_>, Box<[badge::Badge]>); 5] = [
        (
            timeline::Item {
                content: PreEscaped(lang_to_html(&loader.get("adverma-short"))),
                content_long: html! {
                    div style="display:flex; justify-content: space-between;"{
                        p {(lang_to_html(&loader.get("adverma-long")))}
                        (profile_img.render(ImgProps { path_to_root:&page.path_to_root(lang),..Default::default() }))
                    }
                },
                title: "ADVERMA GmbH",
                wide: false,
                year: "2025 - 2026",
            },
            Box::from([]),
        ),
        (
            timeline::Item {
                content: PreEscaped(lang_to_html(&loader.get("sem-4-short"))),
                content_long: PreEscaped(lang_to_html(&loader.get("sem-4-long"))),
                title: "THI - Semester 4",
                wide: true,
                year: "2025",
            },
            Box::from([
                badge::Badge::WDWU,
                badge::Badge::ProduktDesign,
                badge::Badge::PcGraph,
                badge::Badge::PMMI,
                badge::Badge::ProjektManagement,
            ]),
        ),
        (
            timeline::Item {
                content: PreEscaped(lang_to_html(&loader.get("sem-3-short"))),
                content_long: PreEscaped(lang_to_html(&loader.get("sem-3-long"))),
                title: "THI - Semester 3",
                wide: false,
                year: "2024 - 2025",
            },
            Box::from([]),
        ),
        (
            timeline::Item {
                content: PreEscaped(lang_to_html(&loader.get("sem-2-short"))),
                content_long: PreEscaped(lang_to_html(&loader.get("sem-2-long"))),
                title: "THI - Semester 2",
                wide: true,
                year: "2024",
            },
            Box::from([]),
        ),
        (
            timeline::Item {
                content: PreEscaped(lang_to_html(&loader.get("sem-1-short"))),
                content_long: PreEscaped(lang_to_html(&loader.get("sem-1-long"))),
                title: "THI - Semester 1",
                wide: false,
                year: "2023 - 2024",
            },
            Box::from([]),
        ),
    ];

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (timeline_script.render(&page.path_to_root(lang)))
                (timeline_style.render(&page.path_to_root(lang)))
                (default_head(&core_loader.get("Experience"),"//TODO:", page, lang))
                (badge_style.render(&page.path_to_root(lang)))
            }

            body{
                style { (PreEscaped(STYLE)) }
                (header(page, lang))
                main{
                    section.sect.content{(timeline_html(timeline::MarkupProps {
                        heading:"Erfahrung",
                        level: 1,
                        items,
                        start_left:true
                    }))}
                }
                (footer(lang))
                // script{(PreEscaped(SCRIPT))}
            }
        },
    )
}
