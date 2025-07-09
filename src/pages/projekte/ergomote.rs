use crate::{
    components::{footer::footer, head::head, header::header},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"//TODO:"#;

pub const MOD_PATH: &str = module_path!();
pub const META_DATA: ProjectMetadata = ProjectMetadata {
    title_img: link_public!(""),
    name: "Ergomote",
    description: DESCRIPTION,
    category: projekte::Category::Design3D,
    favorite: false,
};

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
