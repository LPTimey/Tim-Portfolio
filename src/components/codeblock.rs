use Props::with_props;
use maud::{html, Markup, PreEscaped};

use crate::{Link, components::Component, link_public};

#[with_props]
pub fn markup<'a>(id: &'static str, data: &'a str, prog_lang: &'a str) -> Markup {
    html! {
        code.codeblock.(format!("language-{prog_lang}")) #(id){
            (PreEscaped(data))
        }
    }
}
pub fn style() -> Link {
    link_public!("components/codeblock.css")
}
pub fn script() -> Link {
    link_public!("components/codeblock.js")
}
pub fn component<'a>() -> Component<MarkupProps<'a>, Link, Link> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
