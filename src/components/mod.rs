use maud::Markup;

use crate::Link;

pub mod footer;
pub mod head;
pub mod header;
pub mod hero;
pub mod hyper_img;
pub mod icon;
pub mod img;
pub mod page;
pub mod phone_border;
pub mod project_card;
pub mod project_table;
pub mod scrolling_img;
pub mod theme_select;
pub mod three_js_setup;
pub mod tooltip;

// Definiere ein Trait
pub trait ScriptType {}
impl ScriptType for Link {}
impl ScriptType for () {}
pub struct Component<T, S: ScriptType> {
    pub html: fn(T) -> Markup,
    pub style: Link,
    pub script: S,
}
