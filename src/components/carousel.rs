use Props::with_props;
use maud::{Markup, PreEscaped, html};

use crate::{
    Link,
    components::{
        Component,
        img::{self, ImgProps},
    },
    include_asset, link_public,
};

#[with_props]
pub fn markup<'a>(id: &'static str, pre_src: &'a str, images: &'a [Link]) -> Markup {
    html! {
        div.carousel #(id) "data-current"=(0){
            ul."carousel-content"{@for (i,img) in images.iter().enumerate(){
                    li "data-index"=(i){picture{
                        (img::img(ImgProps{pre_src,src:*img,..Default::default()}))
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
pub fn style() -> Link {
    link_public!("components/carousel.css")
}
pub fn script() -> Link {
    link_public!("components/carousel.js")
}
pub fn component<'a>() -> Component<MarkupProps<'a>, Link> {
    Component {
        html: markup,
        style: style(),
        script: script(),
    }
}
