use maud::{html, PreEscaped};

use crate::{include_asset, link_public, Link};

const UP_ICON: &str = include_asset!("Material Symbols/vertical_align_top_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg");

pub fn footer() -> maud::Markup {
    html! {
        div.grow{}
        footer #SiteFooter{
            "Kreiert mit Rust, HTML, CSS & JS (+ ThreeJS & highlight.js) von Tim Ruland © 2025"
        }
        a href="#" draggable="false" .btn."secondary-btn".shadow #ReturnToTop aria-label="scroll back to Top"{ (PreEscaped(UP_ICON)) }
    }
}

fn _style() -> Link{
    link_public!("components/footer.css")
}
