use std::{sync::Arc, time::Duration};

use Props::with_props;
use maud::{Markup, PreEscaped, html};

use crate::{
    assets::{
        img::{Img, ImgProps},
        script::Script,
        stylesheet::StyleSheet,
    },
    components::Component,
    include_asset,
};

#[with_props]
pub fn markup(
    id: String,
    class: String,
    pre_src: String,
    images: Vec<Arc<Img>>,
    aspect: f64,
    auto_scroll: Option<Duration>,
    eager: bool,
) -> Markup {
    let auto_scroll = if let Some(auto_scroll) = auto_scroll {
        auto_scroll.as_millis()
    } else {
        0
    };
    html! {
        div class=(format!("carousel {}",class)) #(id) data-current=(0) data-scroll=(auto_scroll) style=(format!("--aspect-ratio:{};",aspect)){
            ul."carousel-content"{
                @for (i,img) in images.iter().enumerate(){
                    li "data-index"=(i){
                        (Arc::clone(img).render(ImgProps{path_to_root:&pre_src,eager,..Default::default()}))
                    }
                }
            }
            div."carousel-dots-wrapper"{
                ul."carousel-dots"{@for (i,_) in images.iter().enumerate(){
                    li."carousel-dot" "data-for"=(i){}
                }}
            }
            button."carousel-button-left".btn."secondary-btn".shadow{(PreEscaped(include_asset!("Material Symbols/play_arrow_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg")))}
            button."carousel-button-right".btn."secondary-btn".shadow{(PreEscaped(include_asset!("Material Symbols/play_arrow_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg")))}
        }
    }
}
pub fn style() -> Arc<StyleSheet> {
    StyleSheet::new("public", "components/carousel.css").unwrap()
}
pub fn script() -> Arc<Script> {
    Script::new("public", "components/carousel.js").unwrap()
}
pub fn component() -> Component<MarkupProps, Arc<StyleSheet>, Arc<Script>> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
