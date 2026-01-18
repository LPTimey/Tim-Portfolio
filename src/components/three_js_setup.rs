use maud::{Markup, PreEscaped, html};
use unic_langid::LanguageIdentifier;

use crate::Page;

pub fn import_map(page: Page, lang: &LanguageIdentifier) -> Markup {
    html! {
        script type="importmap"{(PreEscaped(format!(r#"
        {{
            "imports": {{
                "three": "{path}vendor/three.js/three.js-r182/build/three.module.js",
                "three/addons/": "{path}vendor/three.js/three.js-r182/examples/jsm/"
            }}
        }}
        "#,path=page.path_to_root(lang))))}
    }
}
