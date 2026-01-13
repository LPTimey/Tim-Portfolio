use maud::html;
use unic_langid::LanguageIdentifier;

use crate::{Link, assets::img::ImgProps, components::Component, link_public, projects::ProjectMetadata};
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
        a ."project-card" draggable="false" "in-grid"=(is_in_grid) href=(path_to_root + &data.page.to_href(lang).display().to_string()) {
            div ."pjc-grid" "reactive-color"=(reactive_color) {
                picture{
                    (data.title_img.light().render(ImgProps{path_to_root:&data.page.path_to_root(lang), class:&["light-only"], ..Default::default()}))
                    (data.title_img.dark().render(ImgProps{path_to_root:&data.page.path_to_root(lang), class:&["dark-only"], ..Default::default()}))
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

pub fn component<'a>() -> Component<MarkupProps<'a>, Link, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
