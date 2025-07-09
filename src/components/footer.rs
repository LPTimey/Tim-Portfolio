use maud::{html, PreEscaped};

use crate::include_asset;

const UP_ICON: &str = include_asset!("Material Symbols/vertical_align_top_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg");

pub fn footer() -> maud::Markup {
    html! {
        div.grow{}
        footer #SiteFooter{}
        a href="#" .btn."secondary-btn".shadow #ReturnToTop{ (PreEscaped(UP_ICON)) }
        // a href="#" .btn."secondary-btn".shadow { (PreEscaped(UP_ICON)) }
    }
}
