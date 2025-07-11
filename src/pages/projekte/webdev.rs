use crate::{
    LightDark,
    components::{footer::footer, head::head, header::header},
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Die Geschichte der Entwicklung dieser Website."#;

pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: Box::new(LightDark {
            light: link_public!("assets/WebSite/title-img-light.webp"),
            dark: link_public!("assets/WebSite/title-img-dark.webp"),
        }),
        name: "Website Development",
        description: DESCRIPTION,
        category: projekte::Category::Programmieren,
        favorite: false,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}

pub fn page(page: Page) -> maud::Markup {
    html! {
        (head("WebDev","TODO: Add description",page.path_to_root()))

        body{
            (header(page))
            main{}
            (footer())
        }
    }
}
