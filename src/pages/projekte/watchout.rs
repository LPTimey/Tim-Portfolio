use crate::{
    components::{footer::footer, head::head, header::header},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Eine Uhr und eine App um Menschen mit Demenz und deren Familie zu helfen ihr Leben sorgloser zu leben."#;

pub const MOD_PATH: &str = module_path!();
pub const META_DATA: ProjectMetadata = ProjectMetadata {
    title_img: link_public!(""),
    name: "Watch Out",
    description: DESCRIPTION,
    category: projekte::Category::DMMS,
    favorite: true,
};

pub fn page(page: Page) -> maud::Markup {
    html! {
        (head("Watchout","TODO: Add description",page.path_to_root()))

        body{
            (header(page))
            main{}
            (footer())
        }
    }
}
