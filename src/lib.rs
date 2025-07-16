mod components;
mod pages;

use std::{
    fmt::Display, fs::{self, File}, io::BufReader, ops::Deref, path::{Path, PathBuf}
};

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
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logos/", $path))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link(&'static str);
impl Link {
    pub fn exists(&self) -> bool {
        PathBuf::from(self.0).exists()
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
    pub fn get_img_dimensions_panic(&self)->(usize, usize){
        self.get_img_dimensions().expect("a valid img")
    }
    /// Entfernt führende ../ oder ./ und prependet "public/"
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

/// Entfernt `.` und `..` sauber aus Pfaden (ohne zu canonicalizen)
fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for comp in path.components() {
        use std::path::Component;
        match comp {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            other => result.push(other.as_os_str()),
        }
    }
    result
}
