use maud::{PreEscaped, html};

use crate::{components::{img, Component}, link_public, Link};

use Props::with_props;

#[with_props]
fn markup(img: Link, rows: u8, columns: u8, duration: std::time::Duration) -> maud::Markup {
    let imgs = html! {picture{(img::img(img, "",None,&[],None))}}
        .into_string()
        .repeat(rows as usize * columns as usize);

    html! {
        div."scroll-img" style=(format!("--columns: {columns};--rows: {rows};--duration: {duration}s",duration=duration.as_secs())){
            (PreEscaped(imgs))
        }
    }
}
fn style() -> Link {
    link_public!("components/scroll_img.css")
}

pub fn component() -> Component<MarkupProps, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
