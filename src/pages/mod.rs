pub mod index;
pub mod projekte;

use std::{ops::Deref, path::PathBuf};

use maud::{Markup, html};
use strum::{Display, EnumIter, VariantArray};

use crate::{Link, include_asset, link_public, path_to_root};

fn mod_path_to_href(mod_path: &str) -> Option<PathBuf> {
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

#[derive(Debug, Clone, Copy, Display, EnumIter, VariantArray)]
pub enum Page {
    Home,
    Watchout,
}
impl Page {
    pub fn to_href(self) -> PathBuf {
        match self {
            Page::Home => mod_path_to_href(index::MOD_PATH).unwrap(),
            Page::Watchout => mod_path_to_href(projekte::watchout::MOD_PATH).unwrap(),
        }
    }
    pub fn to_markup(self) -> Markup {
        match self {
            Page::Home => index::page(self),
            Page::Watchout => projekte::watchout::page(self),
        }
    }
    pub fn metadata(self) {
        match self {
            Page::Home => todo!(),
            Page::Watchout => todo!(),
        }
    }
    pub fn path_to_root(self) -> String {
        path_to_root(self.to_href().deref())
    }
    pub fn is_project(self) -> bool {
        todo!()
    }
}
