mod components;
mod pages;
use std::{ops::Deref, path::Path};

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
macro_rules! link_public {
    ($path:expr) => {
        $crate::Link($path)
    };
}

pub struct Link(&'static str);
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
