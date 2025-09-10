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

#[with_props]
pub fn markup(content: Markup, path_to_root: String) -> Markup {
    html! {
        div."phone-border"{
            picture."phone-pic"{
                (img(ImgProps{pre_src:path_to_root,src:link_public!("assets/iPhone Template [Konvertiert] noBG.png"),class:&["phone"], ..Default::default()}))
                div."pic-content"{(content)}
            }
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
