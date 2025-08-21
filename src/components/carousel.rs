use Props::with_props;
use maud::{html, Markup, PreEscaped};

use crate::{
    components::{
        img::{self, ImgProps}, Component
    }, include_asset, link_public, Link
};

#[with_props]
pub fn markup<'a>(id: &'static str, pre_src: &'a str, images: &'a [Link]) -> Markup {
    html! {
        div.carousel #(id){
            ul."carousel-content"{@for (i,img) in images.iter().enumerate(){
                li{picture #(format!("{id}-{i}")){
                    (img::img(ImgProps{pre_src,src:*img,..Default::default()}))
                }}
            }}
            ul."carousel-dots"{@for _ in images{
                li."carousel-dot"{}
            }}
            menu."carousel-buttons"{
                li{a."carousel-button".btn."secondary-btn".shadow{(PreEscaped(include_asset!("Material Symbols/play_arrow_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg")))}}
                li{a."carousel-button".btn."secondary-btn".shadow{(PreEscaped(include_asset!("Material Symbols/play_arrow_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.svg")))}}
            }
        }
    }
}
pub fn style() -> Link {
    link_public!("components/carousel.css")
}
pub fn component<'a>() -> Component<MarkupProps<'a>, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
