use crate::{
    components::{footer::footer, head::head, header::header},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Anpassung des Hochschuldrucker UIs für bessere Les- und Bedienbarkeit ohne den Verlust von Features."#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: Box::new(link_public!((path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path()) + "assets/Screendesign/Drucker/title-img-zoomed.webp").leak())),
        name: "Drucker Touchscreen",
        description: DESCRIPTION,
        category: projekte::Category::Screendesign,
        favorite: true,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}

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
