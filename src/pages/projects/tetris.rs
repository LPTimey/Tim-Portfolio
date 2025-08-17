use std::sync::OnceLock;

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    components::{
        footer::footer, head::default_head, header::header, img, page, project_table::{self, with_sub_heading}, Component
    },
    placeholder_img,
    projects::ProjectMetadata, setup_language_loader,
};

use super::super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "tetris")
}

pub const MOD_PATH: &str = module_path!();
pub fn meta_data(lang: &LanguageIdentifier) -> ProjectMetadata {
    let loader = get_language_loader().select_languages(&[lang]);
    ProjectMetadata {
        page: Page::Tetris,
        title_img: link_public!("assets/Tetris/Title-img.webp").into(),
        name: "Tetris in Arduino & C",
        description: loader.get("description").leak(),
        category: projects::Category::Programmieren,
        favorite: true,
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

    page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head("Tetris","//TODO: Add description",page.path_to_root(lang), lang))
                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
            }

            body{
                (header(page, lang))
                main{
                    section #Hero{
                        picture #HeroImg{(img::img (img::ImgProps {
                                pre_src: page.path_to_root(lang),
                                src: meta_data(lang).title_img.light(),
                                ..Default::default()
                            }))}
                    }
                    (table_html(project_table::MarkupProps {
                        // title: "Tetris auf dem Arduino?".into(),
                        title: with_sub_heading("Tetris auf dem Arduino?","Programmieren"),
                        graphic: html!{
                            picture{
                                (img::img (img::ImgProps {
                                    pre_src: page.path_to_root(lang),
                                    src: meta_data(lang).title_img.light(),
                                    ..Default::default()
                                }))
                            }
                        }.into(),
                        rows:&[
                            (&*core_loader.get("module").leak(), "TMMIP").into(),
                            (&*core_loader.get("period").leak(), format!("{} 2024 - {} 2025",core_loader.get("October"),core_loader.get("February")).leak()).into(),
                            (&*core_loader.get("tools").leak(), "Fritzing, VSCode, ArduinoIDE, C++, git, GitHub").into(),
                            (&*core_loader.get("university").leak(), "Technische Hochschule Ingolstadt").into(),
                        ],
                        text: loader.get("content").leak().into()
                    }))
                    section.sect{
                        h2{(loader.get("hardware"))" & "(loader.get("preparation"))}
                        p {(loader.get("hardware-prep"))}
                        ul{
                            li{(loader.get(""))}
                            li{(loader.get(""))}
                            li{(loader.get(""))}
                            li{(loader.get(""))}
                            li{(loader.get(""))}
                            li{(loader.get(""))}
                        }
                        (img::img(img::ImgProps{pre_src:page.path_to_root(lang),src:link_public!("assets/Tetris/webp/Einzelteile.webp"),..Default::default()}))
                    }
                    section.sect{
                        h2{(loader.get("result"))}
                        p {(loader.get("result-coarse"))}
                    }
                }
                (footer(lang))
            }
        },
    )
}
