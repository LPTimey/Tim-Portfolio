use maud::{PreEscaped, html};

use crate::{Link, components::Component, link_public};

use Props::with_props;

#[with_props]
pub fn markup(title: &'a str, theme: &'a str, description: &'a str, img: &'a str) -> maud::Markup {
    html! {
        div .project-card {
            picture{img src=(img) alt="";}
            h3 { (title) }
            h4 { (theme) }
            p { (description) }
        }
    }
}
pub fn style() -> Link {
    link_public!("components/project_card.css")
}
pub fn script() -> Link {
    link_public!("components/project_card.js")
}
pub fn component() -> Component<fn(MarkupProps<'_>) -> PreEscaped<String>> {

    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
