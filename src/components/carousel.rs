use std::sync::Arc;

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
pub fn markup<'a>(
    id: &'static str,
    pre_src: &'a str,
    images: &'a [Arc<Img>],
    eager: bool,
) -> Markup {
    html! {
        div.carousel #(id) "data-current"=(0){
            ul."carousel-content"{@for (i,img) in images.iter().enumerate(){
                    li "data-index"=(i){picture{
                        (Arc::clone(img).render(ImgProps{path_to_root:pre_src,eager,..Default::default()}))
                    }}
                }
                li."carousel-spacer"{}
            }
            ul."carousel-dots"{@for (i,_) in images.iter().enumerate(){
                li."carousel-dot" "data-for"=(i){}
            }}
            menu."carousel-buttons"{
                li{button."carousel-button".btn."secondary-btn".shadow{(PreEscaped(include_asset!("Material Symbols/play_arrow_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg")))}}
                li{button."carousel-button".btn."secondary-btn".shadow{(PreEscaped(include_asset!("Material Symbols/play_arrow_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg")))}}
            }
        }
    }
}
pub fn style() -> Arc<StyleSheet> {
    StyleSheet::new("public", "components/carousel.css").unwrap()
}
pub fn script() -> Arc<Script> {
    Script::new("public", "components/carousel.js").unwrap()
}
pub fn component<'a>() -> Component<MarkupProps<'a>, Arc<StyleSheet>, Arc<Script>> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
