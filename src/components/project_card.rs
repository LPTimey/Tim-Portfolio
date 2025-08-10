use maud::html;
use unic_langid::LanguageIdentifier;

use crate::{
    Link,
    components::{Component, img},
    link_public,
    projects::ProjectMetadata,
};
use Props::with_props;

#[with_props]
fn markup<'a>(
    data: ProjectMetadata,
    path_to_root: String,
    is_in_grid: bool,
    reactive_color: bool,
    lang: &'a LanguageIdentifier,
) -> maud::Markup {
    html! {
        a ."project-card" draggable="false" "in-grid"=(is_in_grid) href=(path_to_root + &data.path.display().to_string()) {
            div ."pjc-grid" "reactive-color"=(reactive_color) {
                picture{
                    (img::img(data.page.path_to_root(lang),data.title_img.light(), "", None, &["light-only"], None))
                    (img::img(data.page.path_to_root(lang),data.title_img.dark(), "", None, &["dark-only"], None))
                }
                h3 .subhead { (data.name) }
                h4 { (data.category) }
                p { (data.description) }
            }
        }
    }
}
fn style() -> Link {
    link_public!("components/project_card.css")
}

pub fn component<'a>() -> Component<MarkupProps<'a>, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
