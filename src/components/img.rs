use maud::{self, Markup, html};

use crate::Link;

pub fn img(
    src: Link,
    alt: &str,
    id: Option<&str>,
    class: &[&str],
    style: Option<&str>
) -> Markup {
    let img_d = src.get_img_dimensions_panic();
    html! {
        img
            src=(*src)
            alt=(alt)
            width=(img_d.0)
            height=(img_d.1)
            id=(id.unwrap_or_default())
            class=(class.join(" "))
            style=(style.unwrap_or_default())
            loading="lazy"
            draggable="false"
            ;
    }
}
