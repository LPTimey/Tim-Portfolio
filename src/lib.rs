mod color;
mod components;
mod pages;

use i18n_embed::{LanguageLoader, fluent::FluentLanguageLoader};
use image::ImageReader;
pub use pages::*;
use rayon::prelude::*;
use rust_embed::RustEmbed;
use std::{
    fmt::Display,
    ops::Deref,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use unic_langid::langid;

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
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/public/assets/logos/",
            $path
        ))
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
macro_rules! link_public_img {
    ($path: literal) => {{
        // const _: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/public/", $path)); // compileTime check if file exists
        $crate::Img($crate::Link($path))
    }};
    ($path:expr) => {
        $crate::Img($crate::Link($path))
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
pub const SUPPORTED_LANGS: [unic_langid::LanguageIdentifier; 2] =
    [unic_langid::langid!("de-DE"), unic_langid::langid!("en-GB")];
pub static CORE_LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_core_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&CORE_LANGUAGE_LOADER, "core")
}
pub fn setup_language_loader(
    loader: &'static OnceLock<FluentLanguageLoader>,
    domain: &str,
) -> &'static FluentLanguageLoader {
    loader.get_or_init(|| {
        let loader = FluentLanguageLoader::new(domain, langid!("de-DE"));
        let _ = loader.load_languages(&Localizations, &SUPPORTED_LANGS);
        loader
    })
}

pub fn lang_to_html(string: &str) -> String {
    string
        .replace("\\\n", "")
        .replace("\n", "<br />")
        .replace("\\n", "<br />")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Link(&'static str);
impl Link {
    pub fn exists(&self) -> bool {
        self.into_public_path().exists()
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

pub enum Size {
    Mobile = 360,
    MobilePlus = 412,
    HD = 768,
    Tablet = 1024,
    HDPlus = 1920,
    Content = 1260,
    UHD = 2560,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Img(Link);
impl Img {
    const SIZES: [u32; 7] = [
        Size::Mobile as u32,
        Size::MobilePlus as u32,
        Size::HD as u32,
        Size::Tablet as u32,
        Size::HDPlus as u32,
        Size::Content as u32,
        Size::UHD as u32,
    ];
    pub fn get_img_dimensions(&self) -> Option<(usize, usize)> {
        let path = self.0.into_public_path();

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
    pub fn get_img_srcset(image_data: &[(u32, PathBuf)], path_to_root: &str) -> String {
        image_data
            .iter()
            .map(|(size, path)| {
                let clean_path = path.strip_prefix("public").unwrap_or(path);

                let web_path = clean_path.to_string_lossy().replace('\\', "/");

                format!("{}{} {}w", path_to_root, web_path, size)
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
    pub fn sizes_to_disc(&self) -> Option<Vec<(u32, PathBuf)>> {
        if !self.0.exists() {
            return None;
        }

        let original_path = self.0.into_public_path();
        let target_dir = original_path.with_extension("");

        std::fs::create_dir_all(&target_dir).ok()?;
        let ignore_path = target_dir.join(".gitignore");
        if !ignore_path.exists() {
            std::fs::write(ignore_path, "*").ok()?;
        }

        let image = ImageReader::open(&original_path)
            .ok()?
            .with_guessed_format()
            .ok()?
            .decode()
            .ok()?;

        let paths: Vec<(u32, PathBuf)> = Self::SIZES
            .into_par_iter()
            .map(|size| {
                let file_path = target_dir.join(format!(
                    "{}.{}",
                    size,
                    image::ImageFormat::WebP.extensions_str()[0]
                ));

                // Überspringen, falls Datei existiert
                if !file_path.exists() {
                    // Thumbnail erstellen (behält Seitenverhältnis bei, effizienter als resize)
                    let resized = image.thumbnail(size, size);

                    // Speichern
                    let _ = resized.save_with_format(&file_path, image::ImageFormat::WebP);
                }

                (size, file_path)
            })
            .collect();

        Some(paths)
    }
}
impl From<Link> for Img {
    fn from(value: Link) -> Self {
        Self(value)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TabIndex(u8);
impl Deref for TabIndex {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Iterator for TabIndex {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0;
        self.0 = self.0.wrapping_add(1);
        Some(value)
    }
}
