use maud::html;

use crate::{Link, components::Component, link_public, projekte::ProjectMetadata};
use Props::with_props;

#[with_props]
fn markup(
    data: ProjectMetadata,
    path_to_root: String,
    is_in_grid: bool,
    reactive_color: bool,
) -> maud::Markup {
    html! {
        a ."project-card" "in-grid"=(is_in_grid) href=(path_to_root + &data.path.display().to_string()) {
            div ."pjc-grid" "reactive-color"=(reactive_color) {
                picture{
                    img ."light-only" loading="lazy" src=(*data.title_img.light());
                    img ."dark-only" loading="lazy" src=(*data.title_img.dark());
                }
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

pub fn component() -> Component<MarkupProps, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
