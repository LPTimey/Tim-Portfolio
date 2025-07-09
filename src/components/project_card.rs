use maud::{PreEscaped, html};

use crate::{components::Component, link_public, projekte::ProjectMetadata, Link};

use Props::with_props;

// #[with_props]
fn markup(data: ProjectMetadata) -> maud::Markup {
    html! {
        div ."project-card" {
            div ."pjc-grid"{
                picture{img src=(data.title_img) alt="";}
                h3 { (data.name) }
                h4 { (data.category) }
                p { (data.description) }
            }
        }
    }
}
fn style() -> Link {
    link_public!("components/project_card.css")
}
fn script() -> Link {
    link_public!("components/project_card.js")
}
pub fn component() -> Component<fn(ProjectMetadata) -> PreEscaped<String>> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
