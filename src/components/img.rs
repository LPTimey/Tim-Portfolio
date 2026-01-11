use Props::with_props;
use maud::{self, Markup, html};

use crate::Img;

#[with_props(Default)]
pub fn img<'a, T: Into<String> + Default>(
    pre_src: T,
    src: Img,
    alt: &'a str,
    id: Option<&'a str>,
    class: &'a [&'a str],
    style: Option<&'a str>,
) -> Markup {
    let pre_src = pre_src.into();
    let img_d = match src.get_img_dimensions() {
        Some(d) => d,
        None => {
            eprintln!("cant find img {}", *src.0);
            (0, 0)
        }
    };
    let paths = Img::get_img_srcset(&src.sizes_to_disc().unwrap_or_default(), &pre_src);

    html! {
        img
            src=(pre_src+*src.0)
            srcset=(paths)
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
