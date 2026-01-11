use maud::{PreEscaped, html};

use crate::{
    Img, Link, components::{Component, img}, link_public
};

use Props::with_props;

#[with_props]
fn markup(img: Img, rows: u8, columns: u8, duration: std::time::Duration) -> maud::Markup {
    let imgs = html! {picture{(img::img(img::ImgProps{pre_src:"",src:img, ..Default::default()}))}}
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

pub fn component() -> Component<MarkupProps, Link, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
