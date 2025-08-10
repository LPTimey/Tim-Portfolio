pub mod index;
pub mod projects;

use std::{fmt::Display, ops::Deref, path::PathBuf};

use i18n_embed::LanguageLoader;
use maud::{Markup, html};
use strum::{EnumIter, VariantArray};
use unic_langid::{langid, LanguageIdentifier};

use crate::{
    Link, get_core_language_loader, include_asset, link_public, path_to_root,
    projects::ProjectMetadata,
};

pub fn mod_path_to_href(mod_path: &str) -> Option<PathBuf> {
    // Entferne alles bis "pages::"
    let trimmed = match mod_path.find("pages::") {
        Some(idx) => &mod_path[(idx + "pages::".len())..],
        None => return None,
    };

    let mut replaced = trimmed
        .split("::")
        .fold(PathBuf::new(), |buf, string| buf.join(string));
    replaced.set_extension("html");
    Some(replaced)
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

#[derive(Debug, Clone, Copy, EnumIter, VariantArray, PartialEq, Eq)]
pub enum Page {
    Home,
    Projects,
    Watchout,
    Printer,
    Styles,
    Tetris,
    Ergomote,
    WebDev,
}
impl Page {
    pub fn to_href(self, lang: &LanguageIdentifier) -> PathBuf {
        PathBuf::new().join(lang.to_string()).join(match self {
            Page::Home => mod_path_to_href(index::MOD_PATH).unwrap(),
            Page::Projects => mod_path_to_href(projects::index::MOD_PATH).unwrap(),
            Page::Watchout => mod_path_to_href(projects::watchout::MOD_PATH).unwrap(),
            Page::Printer => mod_path_to_href(projects::printer::MOD_PATH).unwrap(),
            Page::Styles => mod_path_to_href(projects::styles_themes::MOD_PATH).unwrap(),
            Page::Tetris => mod_path_to_href(projects::tetris::MOD_PATH).unwrap(),
            Page::Ergomote => mod_path_to_href(projects::ergomote::MOD_PATH).unwrap(),
            Page::WebDev => mod_path_to_href(projects::webdev::MOD_PATH).unwrap(),
        })
    }
    pub fn to_markup(self, lang: &LanguageIdentifier) -> Markup {
        match self {
            Page::Home => index::page(self, lang),
            Page::Projects => projects::index::page(self, lang),
            Page::Watchout => projects::watchout::page(self, lang),
            Page::Printer => projects::printer::page(self, lang),
            Page::Styles => projects::styles_themes::page(self, lang),
            Page::Tetris => projects::tetris::page(self, lang),
            Page::Ergomote => projects::ergomote::page(self, lang),
            Page::WebDev => projects::webdev::page(self, lang),
        }
    }
    pub fn projects() -> Vec<ProjectMetadata> {
        Self::VARIANTS
            .iter()
            .flat_map(|page| ProjectMetadata::try_from(*page))
            .collect()
    }
    pub fn path_to_root(self, lang: &LanguageIdentifier) -> String {
        path_to_root(self.to_href(lang).deref())
    }
    pub fn to_localized_string(self, lang: &LanguageIdentifier) -> String {
        let loader = get_core_language_loader().select_languages(&[lang]);
        match self {
            Page::Home => loader.get("Home").to_string(),
            Page::Projects => loader.get("all-projects").to_string(),
            Page::Watchout => loader.get("Watchout").to_string(),
            Page::Printer => loader.get("Printer").to_string(),
            Page::Styles => loader.get("Styles").to_string(),
            Page::Tetris => loader.get("Tetris").to_string(),
            Page::Ergomote => loader.get("Ergomote").to_string(),
            Page::WebDev => loader.get("WebDev").to_string(),
        }
    }

}
