use maud::PreEscaped;

use crate::{
    components::{
        self, footer::footer, head::default_head, header::header, project_table, Component
    }, placeholder_img, projekte::ProjectMetadata
};

use super::super::*;

const DESCRIPTION: &str = r#"Eine Fernbedienung, die durch komfortable Form, pragmatische Bedienung und bewusst hochwertiger Materialität, die Nutzer zum Halten einlädt und mit ihrer Stabilität überzeugt."#;
const CONTENT: &str = r#""#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: Box::new(link_public!(
            (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path())
                + "assets/Ergomote/render3.png")
                .leak()
        )),
        name: "Ergomote",
        description: DESCRIPTION,
        category: projekte::Category::Design3D,
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

    components::page::page(html! {
        head{
            (default_head("Ergomote","TODO: Add description",page.path_to_root()))

            link rel="stylesheet" href=(page.path_to_root()+*table_style);
        }

        body{
            (header(page))
            main{
                section #Hero{
                    picture #HeroImg{img draggable="false" src=(page.path_to_root()+*meta_data().title_img.light()) alt="";}
                }
                (table_html(project_table::MarkupProps {
                    title: "//TODO:".into(),
                    graphic: html!{
                        picture{
                            img loading="lazy" draggable="false"
                                src=(page.path_to_root()+*link_public!("assets/Ergomote/render.png")) alt="";
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
                                        href="//TODO:" target="_blank"
                                        rel="noopener noreferrer">
                                        Niklas Röthlingshöfer</a>"#)).into(),
                        ("Zeitraum", "Mai 2025 - Juni 2025").into(),
                        ("Tools", "Figma, Blender, Photoshop, git, GitHub").into(),
                        ("Hochschule", "Technische Hochschule Ingolstadt").into(),
                    ],
                    text: CONTENT.into()
                }))
            }
            (footer())
        }
    })
}
