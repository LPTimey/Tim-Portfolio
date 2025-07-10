use crate::{
    components::{footer::footer, head::head, header::header},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Spiele-Entwicklung auf embedded systems mit manueller Input Hardware Eingabe und simpler LED Ausgabe."#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: link_public!(""),
        name: "Tetris in Arduino & C",
        description: DESCRIPTION,
        category: projekte::Category::Programmieren,
        favorite: true,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}

pub fn page(page: Page) -> maud::Markup {
    html! {
        (head("Tetris","TODO: Add description",page.path_to_root()))

        body{
            (header(page))
            main{}
            (footer())
        }
    }
}
