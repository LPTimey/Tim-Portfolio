use std::{fmt::Debug, sync::Arc};

use strum::Display;
use unic_langid::LanguageIdentifier;

use crate::{LightDark, Page, assets::img::Img};

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
impl PartialOrd for Category {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Category {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

#[derive(Debug, Clone, Display)]
pub enum ThemeLink {
    Single(Arc<Img>),
    LightDark(LightDark<Arc<Img>>),
}
impl ThemeLink {
    pub fn light(&self) -> Arc<Img> {
        match self {
            ThemeLink::Single(link) => link.clone(),
            ThemeLink::LightDark(light_dark) => light_dark.light.clone(),
        }
    }
    pub fn dark(&self) -> Arc<Img> {
        match self {
            ThemeLink::Single(link) => link.clone(),
            ThemeLink::LightDark(light_dark) => light_dark.dark.clone(),
        }
    }
}
impl From<Arc<Img>> for ThemeLink {
    fn from(value: Arc<Img>) -> Self {
        Self::Single(value)
    }
}
impl From<LightDark<Arc<Img>>> for ThemeLink {
    fn from(value: LightDark<Arc<Img>>) -> Self {
        Self::LightDark(value)
    }
}

trait SortProjects {
    fn sort_by_name_ref(&mut self) -> &mut Self;
    fn sort_by_name(mut self) -> Self
    where
        Self: Sized,
    {
        let _ = self.sort_by_name_ref();
        self
    }
}

#[derive(Debug)]
pub struct ProjectMetadata {
    pub page: Page,
    pub title_img: ThemeLink,
    pub name: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub favorite: bool,
}
impl ProjectMetadata {
    pub fn try_from(page: Page, lang: &LanguageIdentifier) -> Option<Self> {
        match page {
            Page::Home => None,
            Page::Projects => None,
            Page::Watchout => Some(watchout::meta_data(lang)),
            Page::Printer => Some(printer::meta_data(lang)),
            Page::Styles => Some(styles_themes::meta_data(lang)),
            Page::Tetris => Some(tetris::meta_data(lang)),
            Page::Ergomote => Some(ergomote::meta_data(lang)),
            Page::WebDev => Some(webdev::meta_data(lang)),
            Page::Adverma => None,
            Page::Experience => None,
        }
    }
}
impl SortProjects for Vec<ProjectMetadata> {
    fn sort_by_name_ref(&mut self) -> &mut Self {
        self.sort_by(|a, b| a.name.cmp(b.name));
        self
    }
}
impl SortProjects for [ProjectMetadata] {
    fn sort_by_name_ref(&mut self) -> &mut Self {
        self.sort_by(|a, b| a.name.cmp(b.name));
        self
    }
}
