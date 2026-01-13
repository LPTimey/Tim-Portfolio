use crate::{
    LightDark,
    assets::img::{Img, ImgProps},
    components::{
        self, Component,
        footer::footer,
        head::default_head,
        header::header,
        project_table::{self, with_sub_heading},
    },
    placeholder_img,
    projects::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Die Geschichte der Entwicklung dieser Website."#;
const CONTENT: &str = r#"Die Geschichte der Entwicklung dieser Website."#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data(_lang: &LanguageIdentifier) -> ProjectMetadata {
    ProjectMetadata {
        page: Page::WebDev,
        title_img: LightDark {
            light: Img::new("public", "assets/WebSite/title-img-light.webp", "").unwrap(),
            dark: Img::new("public", "assets/WebSite/title-img-dark.webp", "").unwrap(),
        }
        .into(),
        name: "Website Development",
        description: DESCRIPTION,
        category: projects::Category::Programmieren,
        favorite: false,
    }
}

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();

    let meta_data = meta_data(lang);
    let dark_title_img = meta_data.title_img.dark();
    let light_title_img = meta_data.title_img.light();

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head("WebDev","//TODO: Add description",page, lang))
                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
            }

            body{
                (header(page, lang))
                main{
                    section #Hero{
                        picture #HeroImg{
                            (dark_title_img.render(ImgProps { path_to_root: &page.path_to_root(lang), eager: true, class: &["dark-only"], ..Default::default() }))
                            (light_title_img.render(ImgProps { path_to_root: &page.path_to_root(lang), eager: true, class: &["light-only"], ..Default::default() }))
                        }
                    }
                    (table_html(project_table::MarkupProps {
                        // title: "Webentwicklung: Design und Programmieren".into(),
                        title: with_sub_heading("Webentwicklung","Design & Programmieren"),
                        graphic: html!{
                            picture{
                                img loading="lazy" draggable="false"
                                    src=(placeholder_img!(1200,800)) alt="";
                            }
                        }.into(),
                        rows:&[
                            ("Studienmodul", "Produktdesign").into(),
                            ("Zeitraum", "Februar 2025 - ongoing").into(),
                            ("Tools", "Rust, HTML, CSS, JS, Illustrator, Figma, VSCode, Neovim, git, GitHub").into(),
                            ("Hochschule", "Technische Hochschule Ingolstadt").into(),
                        ],
                        text: CONTENT.into(),
                        long_text: false
                    }))
                }
                (footer(lang))
            }
        },
    )
}
