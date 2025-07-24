use maud::{PreEscaped, Render, html};

use crate::{
    Link,
    components::{Component, img},
    link_public,
};
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
    Link { path_to_root: String, link: Link },
    Markup(PreEscaped<String>),
}
impl Render for Graphic {
    fn render_to(&self, buffer: &mut String) {
        match self {
            Graphic::Link { path_to_root, link } => {
                html! {picture{(img::img(Link((path_to_root.to_owned()+**link).leak()),"",None,&[],None));}}
                    .render_to(buffer);
            }
            Graphic::Markup(pre_escaped) => pre_escaped.render_to(buffer),
        };
    }
}
impl From<PreEscaped<String>> for Graphic {
    fn from(value: PreEscaped<String>) -> Self {
        Graphic::Markup(value)
    }
}
impl<T: Into<String>> From<(T, Link)> for Graphic {
    fn from((path_to_root, link): (T, Link)) -> Self {
        Graphic::Link {
            path_to_root: path_to_root.into(),
            link,
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
fn markup<'a>(title: Content, graphic: Graphic, rows: &'a [Row], text: Content) -> maud::Markup {
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
                div #IntroContent{
                    (text)
                }
            }
        }
    }
}
fn style() -> Link {
    link_public!("components/project_table.css")
}

pub fn component<'a>() -> Component<MarkupProps<'a>, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
