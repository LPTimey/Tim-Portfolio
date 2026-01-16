use std::{collections::HashMap, fmt::Display, sync::Arc};

use Props::with_props;
use maud::{Markup, html};

use crate::{
    Link,
    assets::{img::{Img, ImgProps}, stylesheet::StyleSheet},
    components::Component,
    link_public,
};

#[derive(Default)]
pub struct HyperMap(pub HashMap<String, MapNode>, pub (u32, u32));
pub struct MapNode {
    pub buttons: Vec<(InsetPercent, Href)>,
    pub img: Arc<Img>,
    pub alpha: bool,
    pub default: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InsetPercent {
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
}
impl InsetPercent {
    pub fn new(top: u8, right: u8, bottom: u8, left: u8) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn as_style(&self) -> String {
        let Self {
            top,
            right,
            bottom,
            left,
        } = self;
        format!("top:{top}%;right:{right}%;bottom:{bottom}%;left:{left}%;")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Href {
    Back,
    BackThen(String),
    Specific(String),
}
impl Display for Href {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Href::Back => {
                write!(f, "#Back")
            }
            Href::BackThen(str) => {
                write!(f, "{}:{}", Self::Back, str)
            }
            Href::Specific(str) => {
                write!(f, "{str}")
            }
        }
    }
}

#[with_props]
pub fn markup(map: HyperMap, path_to_root: String, eager: bool) -> Markup {
    html! {
        div ."hyper-img" style=(format!("--aspect:{} / {}",map.1.0,map.1.1)){
            @for page in map.0.iter(){
                div."hi-page-wrapper" for=(page.0){
                    (Arc::clone(&page.1.img).render(ImgProps{
                        path_to_root: &path_to_root,
                        eager,
                        id: Some(page.0),
                        class: &["hi-page"],
                        attrs: &[("data-active",&page.1.default.to_string())],
                        children: Some(html!{
                            @for button in page.1.buttons.iter(){
                                @if !format!("{}",button.1).ends_with(page.0){
                                    button type="button" style=(button.0.as_style()) href=(button.1) aria-label=(format!("link to: {}",button.1)){}
                                }
                            }
                        }),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}

fn style() -> Arc<StyleSheet> {
    StyleSheet::new("public","components/hyper_img.css").unwrap()
}
fn script() -> Link {
    link_public!("components/hyper_img.js")
}

pub fn component() -> Component<MarkupProps, Arc<StyleSheet>, Link> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
