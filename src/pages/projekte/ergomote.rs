use crate::{
    components::{footer::footer, head::head, header::header},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"//TODO:"#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: Box::new(link_public!("")),
        name: "Ergomote",
        description: DESCRIPTION,
        category: projekte::Category::Design3D,
        favorite: false,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}

pub fn page(page: Page) -> maud::Markup {
    html! {
        (head("Ergomote","TODO: Add description",page.path_to_root()))

        body{
            (header(page))
            main{}
            (footer())
        }
    }
}
