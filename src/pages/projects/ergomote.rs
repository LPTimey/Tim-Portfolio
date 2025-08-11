use maud::PreEscaped;

use crate::{
    components::{
        self, Component,
        footer::footer,
        head::default_head,
        header::header,
        img,
        project_table::{self, with_sub_heading},
    },
    projects::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Eine Fernbedienung, die durch komfortable Form, pragmatische Bedienung und bewusst hochwertiger Materialität, die Nutzer zum Halten einlädt und mit ihrer Stabilität überzeugt."#;
const CONTENT: &str = r#""#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        page: Page::Ergomote,
        title_img: link_public!("assets/Ergomote/render3.png").into(),
        name: "Ergomote",
        description: DESCRIPTION,
        category: projects::Category::Design3D,
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
                (default_head("Ergomote","TODO: Add description",page.path_to_root(lang),lang))

                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
                style{
                        (PreEscaped(include_asset!("ergomote.css")))
                    }
            }

            body{
                (header(page,lang))
                main{
                    section #Hero{
                        picture #HeroImg{(img::img (page.path_to_root(lang),meta_data().title_img.light(),"",None,&[],None))}
                    }
                    (table_html(project_table::MarkupProps {
                        // title: "Ergomote".into(),
                        title: with_sub_heading("Ergomote","3D- & Produktdesign"),
                        graphic: html!{
                            picture{
                                (img::img (page.path_to_root(lang),link_public!("assets/Ergomote/render.png"),"",None,&[],None))
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
                            ("Tools", "Figma, Blender, Photoshop, git, GitHub").into(),
                            ("Hochschule", "Technische Hochschule Ingolstadt").into(),
                        ],
                        text: CONTENT.into()
                    }))
                }
                (footer(lang))
            }
        },
    )
}
