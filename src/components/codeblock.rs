use std::sync::Arc;

use Props::with_props;
use maud::{Markup, PreEscaped, html};

use crate::{Link, assets::stylesheet::StyleSheet, components::Component, link_public};

#[with_props]
pub fn markup<'a>(id: &'static str, data: &'a str, prog_lang: &'a str) -> Markup {
    html! {
        code.codeblock.(format!("language-{prog_lang}")) #(id){
            (PreEscaped(data))
        }
    }
}
pub fn style() -> Arc<StyleSheet> {
    StyleSheet::new("public","components/codeblock.css").unwrap()
}
pub fn script() -> Link {
    link_public!("components/codeblock.js")
}
pub fn component<'a>() -> Component<MarkupProps<'a>, Arc<StyleSheet>, Link> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
