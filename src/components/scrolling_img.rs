use std::sync::Arc;

use maud::{html};

use crate::{
    angle::Angle,
    assets::{
        img::{Img, ImgProps}, script::Script,
    },
    components::Component,
};

use Props::with_props;

#[with_props]
fn markup<'a>(
    img: Arc<Img>,
    img_props: ImgProps<'a>,
    zoom: f64,
    angle: Angle,
    speed: f64,
) -> maud::Markup {
    let href = img.web_path(img_props.path_to_root);
    html! {
        canvas."scroll-img" data-href=(href) data-zoom=(zoom) data-angle-rad=(angle.as_rad()) data-speed=(speed){
            (img.render(img_props))
        }
    }
}
fn script() -> Arc<Script> {
    Script::new("public", "components/scroll_img.js").unwrap()
}

pub fn component<'a>() -> Component<MarkupProps<'a>, (), Arc<Script>> {
    Component {
        html: markup,
        style: (),
        script: script(),
    }
}
