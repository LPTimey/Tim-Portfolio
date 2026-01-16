use std::sync::Arc;

use Props::with_props;
use maud::{Markup, PreEscaped, html};

use crate::{
    assets::{script::Script, stylesheet::StyleSheet}, components::{Component, badge}
};

pub struct Item<'a> {
    pub year: &'a str,
    pub wide: bool,
    pub title: &'a str,
    pub content: Markup,
    pub content_long: Markup,
}

#[with_props]
pub fn markup<'a, const S: usize>(
    heading: &'a str,
    level: u8,
    items: [(Item<'a>, Box<[badge::Badge]>); S],
    start_left: bool,
) -> Markup {
    html! {
        div."detail-timeline"{
            @match level {
                1 => h1.hero."dt-title" {(PreEscaped(heading))}
                2 => h2.hero."dt-title" {(PreEscaped(heading))}
                3 => h3.hero."dt-title" {(PreEscaped(heading))}
                4 => h4.hero."dt-title" {(PreEscaped(heading))}
                5 => h5.hero."dt-title" {(PreEscaped(heading))}
                6 => h6.hero."dt-title" {(PreEscaped(heading))}
                _ => p.hero."dt-title" {(PreEscaped(heading))}
            }
            div."dt-items" {
                @for (i,(item,badges)) in items.into_iter().enumerate() {
                    div."dt-item" data-left=((start_left as usize + i) % 2 != 0) {
                        @match level {
                            1 => h2.heading."dt-item-title"{ (item.title) }
                            2 => h3.heading."dt-item-title"{ (item.title) }
                            3 => h4.heading."dt-item-title"{ (item.title) }
                            4 => h5.heading."dt-item-title"{ (item.title) }
                            5 => h6.heading."dt-item-title"{ (item.title) }
                            _ => p.heading."dt-item-title"{ (item.title) }
                        }
                        p."dt-button" {}
                        p.shadow."no-hover"."dt-year".large."fw-bold" data-wide=(item.wide) {
                            (item.year)
                        }
                        div."dt-content".shadow."no-hover" data-open="false"{
                            div."dt-short-content"{
                                p{(item.content)}
                                ul."badge-list"{
                                    @for badge in badges.into_iter() {
                                        li{(badge)}
                                    }
                                }
                                button.link.underline { "Mehr >" }
                            }
                            div."dt-long-content"{
                                p{(item.content_long)}
                                button.link.underline { "< Weniger" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn style() -> Arc<StyleSheet> {
    StyleSheet::new("public", "components/detail-timeline.css").unwrap()
}

pub fn script() -> Arc<Script> {
    Script::new("public", "components/detail-timeline.js").unwrap()
}

pub fn component<const S: usize>() -> Component<MarkupProps<'static, S>, Arc<StyleSheet>, Arc<Script>> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
