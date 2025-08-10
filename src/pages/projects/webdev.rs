use crate::{
    LightDark,
    components::{
        self, Component,
        footer::footer,
        head::default_head,
        header::header,
        img,
        project_table::{self, with_sub_heading},
    },
    placeholder_img,
    projects::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Die Geschichte der Entwicklung dieser Website."#;
const CONTENT: &str = r#"Die Geschichte der Entwicklung dieser Website."#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: LightDark {
            light: link_public!(
                (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path())
                    + "assets/WebSite/title-img-light.webp")
                    .leak()
            ),
            dark: link_public!(
                (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path())
                    + "assets/WebSite/title-img-dark.webp")
                    .leak()
            ),
        }
        .into(),
        name: "Website Development",
        description: DESCRIPTION,
        category: projects::Category::Programmieren,
        favorite: false,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head("WebDev","//TODO: Add description",page.path_to_root(lang), lang))
                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
            }

            body{
                (header(page, lang))
                main{
                    section #Hero{
                        picture #HeroImg{(img::img ("",meta_data().title_img.light(),"",None,&[],None))}
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
                        text: CONTENT.into()
                    }))
                }
                (footer())
            }
        },
    )
}
