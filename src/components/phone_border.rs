use std::path::Path;

use Props::with_props;
use maud::{Markup, html};

use crate::{
    Link, assets::img::{Img, ImgProps}, components::Component, link_public
};

#[with_props]
pub fn markup(content: Markup, path_to_root: String, eager:bool) -> Markup {
    let img = Img::new("public", Path::new("assets").join("iPhone Template [Konvertiert] noBG.png"), "").unwrap();
    html! {
        div."phone-border"{
            (img.render(ImgProps{path_to_root:&path_to_root,class:&["phone"],eager, ..Default::default()}))
            div."phone-content"{(content)}
        }
    }
}
pub fn style() -> Link {
    link_public!("components/phone_border.css")
}

pub fn component() -> Component<MarkupProps, Link, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
