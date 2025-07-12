use maud::{html, Markup, PreEscaped};

use crate::Page;

pub fn import_map(page:Page)->Markup{
    html!{
        script type="importmap"{(PreEscaped(format!(r#"
        {{
            "imports": {{
                "three": "{path}vendor/three.js/three.js r176/build/three.module.js",
                "three/addons/": "{path}vendor/three.js/three.js r176/examples/jsm/"
            }}
        }}
        "#,path=page.path_to_root())))}
    }
}