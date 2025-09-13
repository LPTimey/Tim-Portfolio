use std::{collections::HashMap, fmt::Display};

use Props::with_props;
use maud::{Markup, html};

use crate::{
    Link,
    components::{
        Component,
        img::{ImgProps, img},
    },
    link_public,
};

#[derive(Default)]
pub struct HyperMap(pub HashMap<String, MapNode>);
pub struct MapNode {
    pub buttons: Vec<(InsetPercent, Href)>,
    pub img: Link,
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
pub fn markup(map: HyperMap, path_to_root: String) -> Markup {
    html! {
        div ."hyper-img"{
            @for page in map.0.iter(){
                div."hi-page-wrapper" for=(page.0){
                    picture."hi-page" #(page.0) data-active=(page.1.default){
                        (img(ImgProps{pre_src:path_to_root.clone(),src:page.1.img, ..Default::default()}))
                        @for button in page.1.buttons.iter(){
                            @if !format!("{}",button.1).ends_with(page.0){
                                button type="button" style=(button.0.as_style()) href=(button.1) aria-label=(format!("link to: {}",button.1)){}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn style() -> Link {
    link_public!("components/hyper_img.css")
}
fn script() -> Link {
    link_public!("components/hyper_img.js")
}

pub fn component() -> Component<MarkupProps, Link, Link> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
