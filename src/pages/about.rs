use crate::components::{footer::footer, head::head, header::header};

use super::*;

pub const MOD_PATH: &str = module_path!();

pub fn page(page: Page) -> maud::Markup {
    html! {
        (head("About", "TODO: Add description", page.path_to_root()))
        body{
            (header(page))
            main{}
            (footer())
        }
    }
}
