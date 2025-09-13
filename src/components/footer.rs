use maud::{PreEscaped, html};
use unic_langid::LanguageIdentifier;

use crate::{Link, get_core_language_loader, include_asset, link_public};

const UP_ICON: &str = include_asset!(
    "Material Symbols/vertical_align_top_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg"
);

pub fn footer(lang: &LanguageIdentifier) -> maud::Markup {
    let loader = get_core_language_loader().select_languages(&[lang]);
    html! {
        div.grow{}
        footer #SiteFooter{
            (loader.get("footer"))
        }
        a href="#" draggable="false" .btn."secondary-btn".shadow #ReturnToTop aria-label="scroll back to Top"{ (PreEscaped(UP_ICON)) }
    }
}

fn _style() -> Link {
    link_public!("components/footer.css")
}
