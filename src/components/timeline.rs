use Props::with_props;
use maud::{Markup, PreEscaped, html};

use crate::{Link, components::Component, link_public};

pub struct Item<'a> {
    pub year: &'a str,
    pub wide: bool,
    pub title: &'a str,
    pub content: &'a str,
    pub content_long: &'a str,
}

#[with_props]
pub fn markup<'a>(heading: &'a str, items: &'a [Item<'a>], start_left: bool) -> Markup {
    html! {
        div."detail-timeline"{
            h1.title {(PreEscaped(heading))}
            div."detail-timeline-items" {
                @for (i,item) in items.iter().enumerate() {
                    div."detail-timeline-item" data-left=((start_left as usize + 1 * i) % 2 != 0) {
                        h2.subtitle."dt-item-title"{ (item.title) }
                        p."dt-button" {}
                        p.shadow."no-hover"."dt-year" data-wide=(item.wide) {
                            (item.year)
                        }
                        div."dt-content"{
                            div."dt-short-content"{
                                (item.content)
                                button.link.underline { "Mehr >" }
                            }
                            div."dt-long-content"{
                                (item.content_long)
                                button.link.underline { "< Weniger" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn style() -> Link {
    link_public!("components/detail-timeline.css")
}

pub fn component() -> Component<MarkupProps<'static>, Link, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
