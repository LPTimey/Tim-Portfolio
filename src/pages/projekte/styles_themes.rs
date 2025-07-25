use maud::PreEscaped;

use crate::{
    components::{
        footer::footer, head::default_head, header::header, img, page, project_table::{self, with_sub_heading}, Component
    },
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str =
    r#"Ein Experiment, welches die Wichtigkeit eines Ansprechenden Visual Designs zeigt."#;
const CONTENT: PreEscaped<&'static str> = PreEscaped(
    r#"
Im Rahmen dieses Projekts habe ich einen bestehenden Screen einer App oder Website analysiert, 
nachgebaut und anschließend in drei unterschiedlichen UI-Stilen neugestaltet. 
Ziel war es, verschiedene Designtrends zu untersuchen und deren Wirkung auf unterschiedliche Zielgruppen zu reflektieren. 
Neben der gestalterischen Umsetzung lag der Fokus auf der stilistischen Recherche, 
einer Zielgruppenanalyse sowie der fundierten Begründung des Designprozesses.
"#,
);
pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: link_public!(
            (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path())
                + "assets/Screendesign/Styles/title-img.webp")
                .leak()
        ).into(),
        name: "Themen & Stile",
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

    page::page(
        page.path_to_root(),
        html! {
            head{
                (default_head("Style","TODO: Add description",page.path_to_root()))
                link rel="stylesheet" href=(page.path_to_root()+*table_style);
            }

            body{
                (header(page))
                main{
                    section #Hero{
                        picture #HeroImg{(img::img ("",meta_data().title_img.light(),"",None,&[],None))}
                    }
                    (table_html(project_table::MarkupProps {
                        // title: "UI-Stile im Screendesign".into(),
                        title: with_sub_heading("UI Themen & Stile","Screendesign"),
                        graphic: html!{
                            picture{
                                img loading="lazy" draggable="false" id="OriginalImage"
                                    src=(page.path_to_root()+*link_public!("/assets/Screendesign/Styles/Tim_Ruland_Styles_Screendesign_Original_with_new.webp"))
                                    data-source="https://medium.muz.li/weekly-design-inspiration-368-273380298382" alt="";
                            }
                        }.into(),
                        rows:&[
                            ("Studienmodul", "Gestaltung").into(),
                            ("Zeitraum", "Oktober 2023 - Februar 2024").into(),
                            ("Tools", "Illustrator, git, GitHub").into(),
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
