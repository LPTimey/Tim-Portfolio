pub mod index;
pub mod projekte;

use std::{ops::Deref, path::PathBuf};

use maud::{Markup, html};
use strum::{Display, EnumIter, VariantArray};

use crate::{Link, include_asset, link_public, path_to_root, projekte::ProjectMetadata};

pub fn mod_path_to_href(mod_path: &str) -> Option<PathBuf> {
    // Entferne alles bis "pages::"
    let trimmed = match mod_path.find("pages::") {
        Some(idx) => &mod_path[(idx + "pages::".len())..],
        None => return None,
    };

    let replaced = trimmed.replace("::", "/");

    Some(PathBuf::from(format!("{replaced}.html")))
}

pub const STYLE_CSS: Link = link_public!("style.css");
pub const SCRIPT_MJS: Link = link_public!("script.js");
pub const THEME_JS: Link = link_public!("theme.js");
pub const GIT_HUB_ICON: &str = include_asset!("logos/github.svg");
#[macro_export]
macro_rules! placeholder_img {
    ($width:expr, $height:expr) => {
        Link(concat!(
            "https://placehold.co/",
            stringify!($width),
            "x",
            stringify!($height)
        ))
    };
}

#[derive(Debug, Clone, Copy, Display, EnumIter, VariantArray, PartialEq, Eq)]
pub enum Page {
    Home,
    #[strum(to_string = "Alle Projekte")]
    Projekte,
    Watchout,
    Printer,
    Styles,
    Tetris,
    Ergomote,
    WebDev,
}
impl Page {
    pub fn to_href(self) -> PathBuf {
        match self {
            Page::Home => mod_path_to_href(index::MOD_PATH).unwrap(),
            Page::Projekte => mod_path_to_href(projekte::index::MOD_PATH).unwrap(),
            Page::Watchout => mod_path_to_href(projekte::watchout::MOD_PATH).unwrap(),
            Page::Printer => mod_path_to_href(projekte::printer::MOD_PATH).unwrap(),
            Page::Styles => mod_path_to_href(projekte::styles_themes::MOD_PATH).unwrap(),
            Page::Tetris => mod_path_to_href(projekte::tetris::MOD_PATH).unwrap(),
            Page::Ergomote => mod_path_to_href(projekte::ergomote::MOD_PATH).unwrap(),
            Page::WebDev => mod_path_to_href(projekte::webdev::MOD_PATH).unwrap(),
        }
    }
    pub fn to_markup(self) -> Markup {
        match self {
            Page::Home => index::page(self),
            Page::Projekte => projekte::index::page(self),
            Page::Watchout => projekte::watchout::page(self),
            Page::Printer => projekte::printer::page(self),
            Page::Styles => projekte::styles_themes::page(self),
            Page::Tetris => projekte::tetris::page(self),
            Page::Ergomote => projekte::ergomote::page(self),
            Page::WebDev => projekte::webdev::page(self),
        }
    }
    pub fn projects() -> Vec<ProjectMetadata> {
        Self::VARIANTS
            .iter()
            .flat_map(|page| ProjectMetadata::try_from(*page))
            .collect()
    }
    pub fn path_to_root(self) -> String {
        path_to_root(self.to_href().deref())
    }
    pub fn is_project(self) -> bool {
        todo!()
    }
}
