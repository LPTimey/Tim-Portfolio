use std::sync::Arc;

use maud::{PreEscaped, html};

use crate::{
    Link, assets::img::{Img, ImgProps}, components::Component, link_public
};

use Props::with_props;

#[with_props]
fn markup<'a>(img: Arc<Img>, props: ImgProps<'a>, rows: u8, columns: u8, duration: std::time::Duration) -> maud::Markup {
    let imgs = img.render(props)
        .into_string()
        .repeat(rows as usize * columns as usize);

    html! {
        div."scroll-img" style=(
            format!(
                "--columns: {columns};--rows: {rows};animation: {duration}s scroll-{columns}x{rows} infinite linear;",
                duration=duration.as_secs()
            )){
            (PreEscaped(imgs))

            style{(PreEscaped(format!(
                "@keyframes scroll-{columns}x{rows}{{
                    from {{ transform: translate({end_x}%, 0%); }}

                    to {{ transform: translate(0, {end_y}%); }}
                }}",
                end_x=-2.0 * 100.0/rows as f64,
                end_y=-100.0/rows as f64
            )))}
        }
    }
}
fn style() -> Link {
    link_public!("components/scroll_img.css")
}

pub fn component<'a>() -> Component<MarkupProps<'a>, Link, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
