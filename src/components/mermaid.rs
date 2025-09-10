use maud::{html, Markup, PreEscaped};
use Props::with_props;

use crate::{components::Component, link_public, Link};

#[with_props]
pub fn markup<'a, 'b, 'c, 'd>(name: &'a str, defs: &'b[(&'c str, &'d str)]) -> Markup{
    html!{
        div."diagram-container".mermaid."margin-b-medium" name=(name) id=(name){
            style{
                ".diagram-container svg{ padding-block:1em}"
            }
            @for def in defs{
                script class=(format!("diagram-code {}",def.0)) type="text/plain" {
                    (PreEscaped(def.1/*.replace('\n', "&#10;") */))
                }
            }
        }
    }
}
pub fn script() -> Link{
    link_public!("components/mermaid.js")
}
pub fn component<'a, 'b, 'c, 'd>()->Component<MarkupProps<'a, 'b, 'c, 'd>,(),Link>{
    Component { html: markup, style: (), script: script() }
}