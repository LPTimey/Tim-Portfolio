use Props::with_props;
use maud::{self, Markup, html};

use crate::Link;

#[with_props(Default)]
pub fn img<'a, T: Into<String> + Default>(
    pre_src: T,
    src: Link,
    alt: &'a str,
    id: Option<&'a str>,
    class: &'a [&'a str],
    style: Option<&'a str>,
) -> Markup {
    let img_d = match src.get_img_dimensions() {
        Some(d) => d,
        None => {
            eprintln!("cant find img {}", *src);
            (0, 0)
        }
    };
    html! {
        img
            src=(pre_src.into()+*src)
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
