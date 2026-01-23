use std::{path::Path, sync::Arc};

use Props::with_props;
use maud::{Markup, html};

use crate::{
    assets::{img::{Img, ImgProps}, stylesheet::StyleSheet}, components::Component
};

#[with_props]
pub fn markup(content: Markup, path_to_root: String, eager:bool) -> Markup {
    let img = Img::new("public", Path::new("assets").join("iPhone Template [Konvertiert] noBG.png"), "", true).unwrap();
    html! {
        div."phone-border"{
            (img.render(ImgProps{path_to_root:&path_to_root,class:&["phone"],eager, ..Default::default()}))
            div."phone-content"{(content)}
        }
    }
}
pub fn style() -> Arc<StyleSheet> {
    StyleSheet::new("public", "components/phone_border.css").unwrap()
}

pub fn component() -> Component<MarkupProps, Arc<StyleSheet>, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
