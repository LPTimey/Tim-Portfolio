use crate::{
    LightDark,
    components::{
        self, Component, footer::footer, head::default_head, header::header, project_table,
    },
    placeholder_img,
    projekte::ProjectMetadata,
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
        category: projekte::Category::Programmieren,
        favorite: false,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}

pub fn page(page: Page) -> maud::Markup {
    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();

    components::page::page(
        page.path_to_root(),
        html! {
            head{
                (default_head("WebDev","//TODO: Add description",page.path_to_root()))
                link rel="stylesheet" href=(page.path_to_root()+*table_style);
            }

            body{
                (header(page))
                main{
                    section #Hero{
                        picture #HeroImg{img draggable="false" src=(page.path_to_root()+*meta_data().title_img.light()) alt="";}
                    }
                    (table_html(project_table::MarkupProps {
                        title: "Webentwicklung: Design und Programmieren".into(),
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
