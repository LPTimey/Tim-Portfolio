use maud::PreEscaped;

use crate::{
    components::{Component, footer::footer, head::default_head, header::header, project_table},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Anpassung des Hochschuldrucker UIs für bessere Les- und Bedienbarkeit ohne den Verlust von Features."#;
const CONTENT: PreEscaped<&'static str> = PreEscaped(
    r#"
"#,
);

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: Box::new(link_public!(
            (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path())
                + "assets/Screendesign/Drucker/title-img-zoomed.webp")
                .leak()
        )),
        name: "Drucker Touchscreen",
        description: DESCRIPTION,
        category: projekte::Category::Screendesign,
        favorite: true,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}

pub fn page(page: Page) -> maud::Markup {
    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();

    html! {
        head{
            (default_head("Drucker","TODO: Add description",page.path_to_root()))
            link rel="stylesheet" href=(page.path_to_root()+*table_style);
        }

        body{
            (header(page))
            main{
                section #Hero{
                    picture #HeroImg{img draggable="false" src=(page.path_to_root()+*meta_data().title_img.light()) alt="";}
                }
                (table_html(project_table::MarkupProps {
                    title: "Drucker: Motivation & Generelles".into(),
                    graphic: html!{
                        video controls{
                            source src=("video_href") type="video/mp4";
                            a href=("video_href") type="video/mp4"{}
                        }
                    }.into(),
                    rows:&[
                        ("Studienmodul", "Projekt Gestaltung II").into(),
                        ("Zeitraum", "März 2024 - Juli 2024").into(),
                        ("Tools", "Illustrator, XD, git, GitHub").into(),
                        ("Hochschule", "Technische Hochschule Ingolstadt").into(),
                    ],
                    text: CONTENT.into()
                }))
            }
            (footer())
        }
    }
}
