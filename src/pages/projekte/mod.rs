use strum::Display;

use crate::{Link, Page};

pub mod ergomote;
pub mod index;
pub mod printer;
pub mod styles_themes;
pub mod tetris;
pub mod watchout;
pub mod webdev;

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
pub enum Category {
    DMMS,
    Screendesign,
    #[strum(to_string = "3D Design")]
    Design3D,
    Programmieren,
    ProduktDesign,
}

pub struct ProjectMetadata {
    pub title_img: Link,
    pub name: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub favorite: bool,
}
impl ProjectMetadata {
    pub fn try_from(page: Page) -> Option<Self> {
        match page {
            Page::Home => None,
            Page::Projekte => None,
            Page::Watchout => Some(watchout::META_DATA),
            Page::Printer => Some(printer::META_DATA),
            Page::Styles => Some(styles_themes::META_DATA),
            Page::Tetris => Some(tetris::META_DATA),
            Page::Ergomote => Some(ergomote::META_DATA),
            Page::WebDev => Some(webdev::META_DATA),
        }
    }
}
