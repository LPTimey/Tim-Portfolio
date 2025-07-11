use std::path::PathBuf;

use strum::Display;

use crate::{LightDark, Link, Page};

pub mod ergomote;
pub mod index;
pub mod printer;
pub mod styles_themes;
pub mod tetris;
pub mod watchout;
pub mod webdev;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Category {
    DMMS,
    Screendesign,
    #[strum(to_string = "3D Design")]
    Design3D,
    Programmieren,
    ProduktDesign,
}

pub trait TitleImg {
    fn light(&self) -> Link;
    fn dark(&self) -> Link;
}
impl TitleImg for Link {
    fn dark(&self) -> Link {
        *self
    }
    fn light(&self) -> Link {
        *self
    }
}
impl TitleImg for LightDark<Link> {
    fn light(&self) -> Link {
        self.light
    }

    fn dark(&self) -> Link {
        self.dark
    }
}
impl TitleImg for (Link, Link) {
    fn light(&self) -> Link {
        self.0
    }
    fn dark(&self) -> Link {
        self.1
    }
}

pub struct ProjectMetadata {
    pub path: PathBuf,
    pub title_img: Box<dyn TitleImg>,
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
