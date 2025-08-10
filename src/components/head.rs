use std::path::PathBuf;

use maud::{PreEscaped, html};
use strum::IntoEnumIterator;
use unic_langid::LanguageIdentifier;

use crate::{Page, SCRIPT_MJS, STYLE_CSS};

pub fn default_head(
    title: &str,
    description: &str,
    path_to_root: impl Into<PathBuf>,
    lang: &LanguageIdentifier,
) -> maud::Markup {
    let path_to_root: PathBuf = path_to_root.into();
    let path_to_root = path_to_root.to_string_lossy();
    html! {
        meta charset="UTF-8";
        script{
            (PreEscaped(format!("window.pathToRoot='{}';",path_to_root)))
        }
        link rel="stylesheet" async href=(path_to_root.clone() + *STYLE_CSS );
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        meta name="description" content=(description);
        // link rel="stylesheet" href=(path_to_root.clone() + SETUP_CSS );
        title{(title)}
        script type="module" src=(path_to_root.clone() + *SCRIPT_MJS){}
        link rel="shortcut icon" defer href=(path_to_root.clone()+"assets/Lebenslauf/schönes bild klein@0,25x.png") type="image/x-icon";

        @for page in Page::iter() {
            link rel="prefetch" defer href=(path_to_root.to_string()+&page.to_href(lang).display().to_string());
        }
    }
}
