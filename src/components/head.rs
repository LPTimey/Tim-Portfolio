use maud::{PreEscaped, html};
use strum::IntoEnumIterator;
use unic_langid::LanguageIdentifier;

use crate::{Page, SUPPORTED_LANGS, assets::{script::Script, stylesheet::StyleSheet}};

pub fn default_head(
    title: &str,
    description: &str,
    page: Page,
    lang: &LanguageIdentifier,
) -> maud::Markup {
    let path_to_root = page.path_to_root(lang);
    let style = StyleSheet::new("public", "style.css").unwrap();
    let script = Script::new("public", "script.js").unwrap();
    html! {
        meta charset="UTF-8";
        script{
            (PreEscaped(format!("window.pathToRoot='{}';",path_to_root)))
        }
        (style.render(&path_to_root))
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        meta name="description" content=(description);
        // link rel="stylesheet" href=(path_to_root.clone() + SETUP_CSS );
        title{(title)}
        (script.render(&path_to_root))
        link rel="shortcut icon" href=(path_to_root.clone()+"assets/Lebenslauf/schönes bild klein@0,25x.png") type="image/x-icon";

        @for page in Page::iter() {
            link rel="prefetch" href=(path_to_root.to_string()+&page.to_href(lang).components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/"));
        }
        @for langx in SUPPORTED_LANGS {
            @if langx.language != lang.language {
                link rel="prefetch" href=(path_to_root.to_string()+&page.to_href(&langx).components()
                    .map(|c| c.as_os_str().to_string_lossy())
                    .collect::<Vec<_>>()
                    .join("/")) as="document";
            }
        }
    }
}
