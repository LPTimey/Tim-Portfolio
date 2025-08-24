use Props::with_props;
use maud::{Markup, html};

use crate::{Link, components::Component, link_public};

#[with_props]
pub fn markup<'a>(id: &'static str, data: &'a str, prog_lang: &'a str) -> Markup {
    html! {
        code.codeblock.(format!("language-{prog_lang}")) #(id){
            (data)
        }
    }
}
pub fn style() -> Link {
    link_public!("components/codeblock.css")
}
pub fn script() -> Link {
    link_public!("components/codeblock.js")
}
pub fn component<'a>() -> Component<MarkupProps<'a>, Link> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
