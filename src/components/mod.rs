use crate::Link;

pub mod footer;
pub mod head;
pub mod header;
pub mod page;
pub mod project_card;
pub mod theme_select;
pub mod hero;

pub struct Component<T> {
    pub html: T,
    pub style: Link,
    pub script: Link,
}
