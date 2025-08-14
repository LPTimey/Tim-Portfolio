mod components;
mod pages;

use std::{
    fmt::Display,
    ops::Deref,
    path::{Path, PathBuf}, sync::OnceLock,
};
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use unic_langid::langid;
// use i18n_embed_fl::fl;
use rust_embed::RustEmbed;
use image::ImageReader;
pub use pages::*;

#[macro_export]
macro_rules! include_asset {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path))
    };
}
#[macro_export]
macro_rules! include_public {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/public/", $path))
    };
}
#[macro_export]
macro_rules! include_logo {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/public/assets/logos/", $path))
    };
}

#[macro_export]
macro_rules! link_public {
    ($path: literal) => {{
        // const _: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/public/", $path)); // compileTime check if file exists
        $crate::Link($path)
    }};
    ($path:expr) => {
        $crate::Link($path)
    };
}
#[macro_export]
macro_rules! link_logo {
    ($path:literal) => {{
        // const _: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/public/assets/logos/", $path));
        $crate::Link(concat!("assets/logos/",$path))
    }};
}

#[derive(RustEmbed)]
#[folder = "i18n/"]
pub struct Localizations;
pub const SUPPORTED_LANGS: [unic_langid::LanguageIdentifier;2] =
    [unic_langid::langid!("de-DE"), unic_langid::langid!("en-GB")];
pub static CORE_LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_core_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&CORE_LANGUAGE_LOADER, "core")
}
pub fn setup_language_loader(loader:&'static OnceLock<FluentLanguageLoader>, domain: &str) -> &'static FluentLanguageLoader {
    loader.get_or_init(|| {
        let loader = FluentLanguageLoader::new(domain, langid!("de-DE"));
        let _ = loader.load_languages(&Localizations, &SUPPORTED_LANGS);
        loader
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link(&'static str);
impl Link {
    pub fn exists(&self) -> bool {
        self.into_public_path().exists()
    }
    pub fn get_img_dimensions(&self) -> Option<(usize, usize)> {
        let path = self.into_public_path();

        if !path.exists() {
            return None;
        }

        let file = std::fs::File::open(path).ok()?;
        let reader = std::io::BufReader::new(file);

        let image = ImageReader::new(reader)
            .with_guessed_format()
            .ok()?
            .into_dimensions()
            .ok()?;

        Some((image.0 as usize, image.1 as usize))
    }
    pub fn get_img_dimensions_panic(&self) -> (usize, usize) {
        self.get_img_dimensions().expect("a valid img")
    }
    /// Entfernt führende ../ oder ./ und prepend "public/"
    pub fn into_public_path(&self) -> PathBuf {
        let mut cleaned = self.0;

        // Entferne führende "../" oder "./"
        while cleaned.starts_with("../") {
            cleaned = &cleaned[3..];
        }
        if cleaned.starts_with("./") {
            cleaned = &cleaned[2..];
        }

        Path::new("public").join(cleaned)
    }
}
impl Deref for Link {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<Link> for &'static str {
    fn from(val: Link) -> Self {
        val.0
    }
}
impl From<&'static str> for Link {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LightDark<T> {
    pub light: T,
    pub dark: T,
}

pub fn path_to_root(page_path: &Path) -> String {
    let depth = page_path
        .parent() // z. B. "projects"
        .map(|p| p.components().count())
        .unwrap_or(0);

    if depth == 0 {
        "./".to_string()
    } else {
        "../".repeat(depth)
    }
}

pub fn capitalize(str: &str) -> String {
    str.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(f) => f.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
