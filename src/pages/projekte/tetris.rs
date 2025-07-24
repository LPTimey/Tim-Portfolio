use maud::PreEscaped;

use crate::{
    components::{footer::footer, head::default_head, header::header, img, page, project_table, Component}, placeholder_img, projekte::ProjectMetadata
};

use super::super::*;

const DESCRIPTION: &str = r#"Spiele-Entwicklung auf embedded systems mit manueller Input Hardware Eingabe und simpler LED Ausgabe."#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: link_public!(
            (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path())
                + "assets/Tetris/Title-img.webp")
                .leak()
        ).into(),
        name: "Tetris in Arduino & C",
        description: DESCRIPTION,
        category: projekte::Category::Programmieren,
        favorite: true,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}
const CONTENT: PreEscaped<&'static str> = PreEscaped(
    r#"
Im Rahmen eines Hackathons an der Hochschule habe ich eine minimalistische, 
aber voll spielbare Version von Tetris für den Arduino entwickelt. 
Die erste Version entstand auf dem Arduino Uno R4, später folgte die Portierung auf den Uno Rev3, 
was einige technische Änderungen nötig machte.
"#,
);

pub fn page(page: Page) -> maud::Markup {
    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();


    page::page(page.path_to_root(),html! {
        head{
            (default_head("Tetris","//TODO: Add description",page.path_to_root()))
            link rel="stylesheet" href=(page.path_to_root()+*table_style);
        }

        body{
            (header(page))
            main{
                section #Hero{
                    picture #HeroImg{(img::img (Link((page.path_to_root()+*meta_data().title_img.light()).leak()),"",None,&[],None))}
                }
                (table_html(project_table::MarkupProps {
                    title: "Tetris auf dem Arduino?".into(),
                    graphic: html!{
                        picture{
                            img loading="lazy" draggable="false"
                                src=(placeholder_img!(600,400)) alt="";
                        }
                    }.into(),
                    rows:&[
                        ("Studienmodul", "TMMIP").into(),
                        ("Zeitraum", "Oktober 2024 - Februar 2025").into(),
                        ("Tools", "Fritzing, VSCode, ArduinoIDE, C++, git, GitHub").into(),
                        ("Hochschule", "Technische Hochschule Ingolstadt").into(),
                    ],
                    text: CONTENT.into()
                }))
            }
            (footer())
        }
    })
}
