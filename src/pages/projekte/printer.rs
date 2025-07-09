use crate::{
    components::{footer::footer, head::head, header::header},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Anpassung des Hochschuldrucker UIs für bessere Les- und Bedienbarkeit ohne den Verlust von Features."#;

pub const MOD_PATH: &str = module_path!();
pub const META_DATA: ProjectMetadata = ProjectMetadata {
    title_img: link_public!(""),
    name: "Drucker Touchscreen",
    description: DESCRIPTION,
    category: projekte::Category::Screendesign,
    favorite: true,
};

pub fn page(page: Page) -> maud::Markup {
    html! {
        (head("Drucker","TODO: Add description",page.path_to_root()))

        body{
            (header(page))
            main{}
            (footer())
        }
    }
}
