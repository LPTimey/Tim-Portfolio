use maud::{self, Markup, html};

use crate::Link;

pub fn img(
    pre_src: impl Into<String>,
    src: Link,
    alt: &str,
    id: Option<&str>,
    class: &[&str],
    style: Option<&str>
) -> Markup {
    let img_d = match src.get_img_dimensions(){
        Some(d) => d,
        None => {
            eprintln!("cant find img {}",*src);
            (0,0)
        },
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
