use std::sync::Arc;

use maud::{PreEscaped, Render, html};

use crate::{assets::{img::{Img, ImgProps}, stylesheet::StyleSheet}, components::Component};
use Props::with_props;

#[derive(Debug)]
pub enum Content {
    Str(&'static str),
    Markup(PreEscaped<String>),
}
impl Render for Content {
    fn render_to(&self, buffer: &mut String) {
        match self {
            Content::Str(str) => html! {(str)}.render_to(buffer),
            Content::Markup(pre_escaped) => pre_escaped.render_to(buffer),
        };
    }
}
impl From<&'static str> for Content {
    fn from(s: &'static str) -> Self {
        Content::Str(s)
    }
}
impl From<&'static mut str> for Content {
    fn from(s: &'static mut str) -> Self {
        Content::Str(s)
    }
}

impl From<PreEscaped<String>> for Content {
    fn from(p: PreEscaped<String>) -> Self {
        Content::Markup(p)
    }
}
impl From<PreEscaped<&str>> for Content {
    fn from(p: PreEscaped<&str>) -> Self {
        Content::Markup(PreEscaped(p.0.to_string()))
    }
}

pub struct Row {
    pub title: &'static str,
    pub content: Content,
}
impl Render for Row {
    fn render_to(&self, buffer: &mut String) {
        html! {tr{
            th scope="row"{(self.title)}
            td{(self.content)}
        }}
        .render_to(buffer);
    }
}
impl<T: Into<&'static str>, C: Into<Content>> From<(T, C)> for Row {
    fn from(value: (T, C)) -> Self {
        Row {
            title: value.0.into(),
            content: value.1.into(),
        }
    }
}

pub enum Graphic {
    Link { path_to_root: String, img: Arc<Img> },
    Markup(PreEscaped<String>),
}
impl Render for Graphic {
    fn render(&self) -> PreEscaped<String> {
        match self {
            Graphic::Link { path_to_root, img } => {
                img.clone().render(ImgProps{path_to_root,..Default::default()})
            }
            Graphic::Markup(pre_escaped) => pre_escaped.clone(),
        }
    }
}
impl From<PreEscaped<String>> for Graphic {
    fn from(value: PreEscaped<String>) -> Self {
        Graphic::Markup(value)
    }
}
impl<T: Into<String>> From<(T, Arc<Img>)> for Graphic {
    fn from((path_to_root, img): (T, Arc<Img>)) -> Self {
        Graphic::Link {
            path_to_root: path_to_root.into(),
            img,
        }
    }
}

pub fn with_sub_heading(title: &str, sub: &str) -> Content {
    html! {
        span."fw-normal"."ui-small"."lh-tight".block{
            (sub)
        }
        (title)
    }
    .into()
}

#[with_props]
fn markup<'a>(
    title: Content,
    graphic: Graphic,
    rows: &'a [Row],
    text: Content,
    long_text: bool,
) -> maud::Markup {
    html! {
        section.sect.content #Intro{
            div #GeneralInfo{
                h2.heading #IntroTitle{(title)}
                div #Graphic{
                    (graphic)
                }
                table id="TlDrTable" role="presentation" aria-label="Projektinformationen"{
                    tbody{
                        @for row in rows{
                            (row)
                        }
                    }
                }
                div #IntroContent data-long=(long_text){
                    ((match text{
                        Content::Str(str) => PreEscaped(str.to_owned()),
                        Content::Markup(pre_escaped) => pre_escaped,
                    }))
                }
            }
        }
    }
}
fn style() -> Arc<StyleSheet> {
    StyleSheet::new("public", "components/project_table.css").unwrap()
}

pub fn component<'a>() -> Component<MarkupProps<'a>, Arc<StyleSheet>, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
