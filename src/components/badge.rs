use Props::with_props;
use maud::{Markup, PreEscaped, html};

use crate::{color::Color, Link, components::Component, link_public};

#[with_props]
pub fn markup(content: PreEscaped<String>,color:Color) -> Markup {
    html! {
        span.badge style=(format!(r#"
        --color:{}
        "#,color.to_css().into_string())){
            (content)
        }
    }
}
pub fn style() -> Link {
    link_public!("components/badge.css")
}

pub fn component() -> Component<MarkupProps, Link, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}