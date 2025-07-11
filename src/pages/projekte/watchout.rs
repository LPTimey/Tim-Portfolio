use crate::{
    components::{footer::footer, head::head, header::header},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Eine Uhr und eine App um Menschen mit Demenz und deren Familie zu helfen ihr Leben sorgloser zu leben."#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: Box::new(
            link_public!(
                (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path()) + 
                "assets/Design der Mensch Maschine Schnittstelle/WatchOut/title-img-flipp-bg.webp")
                .leak()
            )
        ),
        name: "Watch Out",
        description: DESCRIPTION,
        category: projekte::Category::DMMS,
        favorite: true,
        path: mod_path_to_href(MOD_PATH).expect("A valid path")
    }
}

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
