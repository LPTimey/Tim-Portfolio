use Props::with_props;
use maud::{Markup, html};

use crate::{
    Link,
    components::{Component, img::img},
    link_public,
};

#[with_props]
pub fn markup(content: Markup, path_to_root: String) -> Markup {
    html! {
        div."phone-border"{
            picture."phone-pic"{
                (img(Link((path_to_root+*link_public!("assets/iPhone Template [Konvertiert] noBG.png")).leak()), "", None, &["phone"], None))
                div."pic-content"{(content)}
            }
        }
    }
}
pub fn style() -> Link {
    link_public!("components/phone_border.css")
}

pub fn component() -> Component<MarkupProps, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
