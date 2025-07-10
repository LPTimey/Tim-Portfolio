use std::path::PathBuf;

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
    pub path: PathBuf,
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
            Page::Watchout => Some(watchout::meta_data()),
            Page::Printer => Some(printer::meta_data()),
            Page::Styles => Some(styles_themes::meta_data()),
            Page::Tetris => Some(tetris::meta_data()),
            Page::Ergomote => Some(ergomote::meta_data()),
            Page::WebDev => Some(webdev::meta_data()),
        }
    }
}
