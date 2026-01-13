use std::sync::OnceLock;

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    components::{
        self, Component,
        footer::footer,
        head::default_head,
        header::header,
        icon::{Icon, IconToMarkup},
        img,
        project_table::{self, with_sub_heading},
        three_js_setup::import_map,
        tooltip,
    },
    include_public,
    projects::ProjectMetadata,
    setup_language_loader,
};

use super::super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "ergomote")
}

// const DESCRIPTION: &str = r#"Eine Fernbedienung, die durch komfortable Form, pragmatische Bedienung und bewusst hochwertiger Materialität, die Nutzer zum Halten einlädt und mit ihrer Stabilität überzeugt."#;
// const CONTENT: &str = r#""#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data(lang: &LanguageIdentifier) -> ProjectMetadata {
    let loader = get_language_loader().select_languages(&[lang]);
    ProjectMetadata {
        page: Page::Ergomote,
        title_img: link_public!("assets/Ergomote/render3.png").into(),
        name: "Ergomote",
        description: loader.get("description").leak(),
        category: projects::Category::Design3D,
        favorite: false,
    }
}

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let loader = get_language_loader().select_languages(&[lang]);
    let core_loader = get_core_language_loader().select_languages(&[lang]);

    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (import_map(page, lang))
                script type="module" src=(page.path_to_root(lang)+*link_public!("ergomote.js")){}
                (default_head("Ergomote",&loader.get("description"),page,lang))

                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
                link rel="stylesheet" href=(page.path_to_root(lang) + *tooltip::style() );
                style{
                    (PreEscaped(include_asset!("ergomote.css")))
                }
            }

            body{
                (header(page,lang))
                main{
                    section #Hero{
                        picture #HeroImg{(img::img (img::ImgProps {
                                pre_src: page.path_to_root(lang),
                                src: meta_data(lang).title_img.light(),
                                ..Default::default()
                            }))}
                    }
                    (table_html(project_table::MarkupProps {
                        // title: "Ergomote".into(),
                        title: with_sub_heading("Ergomote","3D- & Produktdesign"),
                        graphic: html!{
                            picture{
                                (img::img (img::ImgProps {
                                    pre_src: page.path_to_root(lang),
                                    src: link_public!("assets/Ergomote/render.png"),
                                    ..Default::default()
                                }))
                            }
                        }.into(),
                        rows:&[
                            ("Studienmodul", "Produktdesign").into(),
                            ("Team", PreEscaped(r#"<a class="link link-active underline"
                                        href="https://mangonssen.github.io/portfolio/" target="_blank"
                                        rel="noopener noreferrer">
                                        Marc Obst</a>,
                                    Tim Ruland,
                                    <a class="link link-active underline"
                                        href="https://niiiicolaas.github.io/Nicolas-Weber-Portfolio/" target="_blank"
                                        rel="noopener noreferrer">
                                        Nicolas Weber</a>
                                    & <a class="link link-active underline"
                                        href="https://github.com/niroet" target="_blank"
                                        rel="noopener noreferrer">
                                        Niklas Röthlingshöfer</a>"#)).into(),
                            ("Zeitraum", "Mai 2025 - Juni 2025").into(),
                            (&*core_loader.get("tools").leak(), html!{ul."icon-row"{([
                                Icon::Figma,
                                Icon::Blender,
                                Icon::Photoshop,
                                Icon::Git,
                                Icon::GitHub
                                ].to_markup(&page.path_to_root(lang)))}}).into(),
                            ("Hochschule", "Technische Hochschule Ingolstadt").into(),
                        ],
                        text: (&*loader.get("content").leak()).into(),
                        long_text: false
                    }))
                    section.sect."accent-background".content style="
                        --accent-bg-c: var(--black);
                        --bg-light: #FBB56A;
                        --bg-normal: #de9446ff;
                        --bg-dark: #c97c2aff;
                        --blob-scale: 70%;
                        --blur: 0.75rem;
                        --op-max:0.2;"{
                        div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}

                        canvas #ErgomoteInfoCanvas width="1200" height="500" style="/*outline:2px solid black;*/ width:100%;"{}

                        div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                    }
                    section.sect{}
                }
                (footer(lang))
            }
        },
    )
}
