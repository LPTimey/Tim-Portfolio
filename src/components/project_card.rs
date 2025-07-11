use maud::html;

use crate::{Link, components::Component, link_public, projekte::ProjectMetadata};
use Props::with_props;

#[with_props]
fn markup(data: ProjectMetadata, path_to_root: String) -> maud::Markup {
    let _ = "";
    let _ = data.title_img;
    html! {
        a ."project-card" href=(path_to_root + &data.path.display().to_string()) {
            div ."pjc-grid"{
                picture{img src=("data.title_img") alt="";}
                div .info{
                    h3 { (data.name) }
                    h4 { (data.category) }
                    p { (data.description) }
                }
            }
        }
    }
}
fn style() -> Link {
    link_public!("components/project_card.css")
}

pub fn component() -> Component<MarkupProps, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
