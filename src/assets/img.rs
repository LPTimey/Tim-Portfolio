use std::{
    io,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use image::{DynamicImage, GenericImageView, ImageFormat, imageops::FilterType};
use maud::{Markup, PreEscaped, html};
use rayon::prelude::*;

use crate::{canonicalize_web_path, needs_copy};

use super::{Asset, register_seen_or_get, register_used};

#[derive(Debug)]
pub struct Img {
    /// Filesystem-Prefix, z. B. ./public
    pub prefix: PathBuf,

    /// Web-relativer Pfad, z. B. assets/img/profile.jpg
    pub path: PathBuf,

    /// Fallback alt (kann beim Render überschrieben werden)
    pub alt: String,

    /// Lazy Cache für (width, height)
    dimensions: OnceLock<(u32, u32)>,
}

impl Img {
    pub const SIZES: [u16; 6] = [360, 768, 1024, 1280, 1920, 2560];
    /// last = Fallback
    pub const FORMATS: [ImageFormat; 2] = [
        // Avif slow AF to encode
        // ImageFormat::Avif,
        ImageFormat::WebP,
        ImageFormat::Png,
    ];

    pub fn new(
        prefix: impl Into<PathBuf>,
        path: impl Into<PathBuf>,
        alt: impl Into<String>,
    ) -> Result<Arc<Self>, io::Error> {
        let prefix = prefix.into();
        let path = path.into();

        let full_path = prefix.join(&path);

        if !full_path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Datei existiert nicht: {}", full_path.display()),
            ));
        }

        let candidate = Arc::new(Self {
            prefix: prefix,
            path: path,
            alt: alt.into(),
            dimensions: OnceLock::new(),
        });

        // Registriere oder hole vorhandenes Asset
        let registered: Arc<dyn Asset> = register_seen_or_get(candidate.clone() as Arc<dyn Asset>);

        Ok(Arc::downcast::<Img>(registered).unwrap_or(candidate))
    }

    pub fn render(self: Arc<Self>, props: ImgProps<'_>) -> Markup {
        // automatische Registrierung beim Rendern
        register_used(&(self.clone() as Arc<dyn Asset>));

        let (width, height) = self.get_dimensions().unwrap();
        let (last, rest) = Self::FORMATS.split_last().unwrap();

        PreEscaped(format!(
            r#"<picture id='{id}' class='{class}' style='{style}' {attrs}>{html}</picture>"#,
            id = props.id.unwrap_or_default(),
            class = (props.class.join(" ")),
            style = (props.style.unwrap_or_default()),
            attrs = props
                .attrs
                .iter()
                .map(|(name, value)| format!(r#"{name}="{value}""#))
                .collect::<Vec<_>>()
                .join(" "),
            html = html! {
                @for format in rest.iter() {
                    source
                        type=(format.to_mime_type())
                        width=(width)
                        height=(height)
                        sizes=(props.sizes)
                        srcset=(self.srcset(props.path_to_root,*format));
                }

                img
                    src=(self.web_path(props.path_to_root))
                    width=(width)
                    height=(height)
                    sizes=(props.sizes)
                    srcset=(self.srcset(props.path_to_root,*last))
                    alt=(if let Some(alt)=props.alt{alt}else {&self.alt})
                    decoding="async"
                    loading=(if props.eager {"eager"} else{"lazy"})
                    draggable="false";

                @if let Some(children) = props.children{
                    (children)
                }
            }
            .to_owned()
            .0
        ))
    }

    /// Vollständiger Filesystem-Pfad (für Optimierung / Lesen)
    pub fn full_path(&self) -> PathBuf {
        self.prefix.join(&self.path)
    }

    pub fn copy_path(&self, prefix: impl Into<PathBuf>) -> PathBuf {
        prefix.into().join(&self.path)
    }
    /// Web-Pfad (HTML-safe, immer `/`)
    pub fn web_path(&self, path_to_root: &str) -> String {
        let path = format!(
            "{path_to_root}/{}",
            self.path.to_string_lossy().replace('\\', "/")
        );
        canonicalize_web_path(&path)
    }

    /// Dateiname ohne Extension
    fn file_stem(&self) -> &str {
        self.path.file_stem().and_then(|s| s.to_str()).unwrap_or("")
    }

    /// Verzeichnis + Dateiname ohne Extension (Web-Pfad)
    fn web_base_path(&self) -> String {
        match self.path.parent() {
            Some(parent) => {
                let mut p = parent.to_string_lossy().replace('\\', "/");
                if !p.is_empty() {
                    p.push('/');
                }
                p.push_str(self.file_stem());
                p
            }
            None => self.file_stem().to_string(),
        }
    }
    /// ../../assets/img/foo/360.webp
    fn processed_web_url(&self, path_to_root: &str, size: u16, format: ImageFormat) -> String {
        let path = format!(
            "{}/{}",
            path_to_root.trim_end_matches('/'),
            self.processed_web_path(size, format)
        );
        canonicalize_web_path(&path)
    }
    /// Verzeichnis + Dateiname + Size + Format (Web-Pfad)
    /// assets/img/foo/360.webp
    fn processed_web_path(&self, size: u16, format: ImageFormat) -> String {
        let ext = format.extensions_str()[0];
        let path = format!("{}/{size}.{ext}", self.web_base_path());
        canonicalize_web_path(&path)
    }
    /// Filesystem-Pfad für das verarbeitete Bild
    fn processed_fs_path(&self, prefix: &Path, size: u16, format: ImageFormat) -> PathBuf {
        prefix.join(self.processed_web_path(size, format))
    }

    pub fn get_dimensions(&self) -> Result<(u32, u32), image::ImageError> {
        if let Some(dim) = self.dimensions.get() {
            return Ok(*dim);
        }

        let dim = image::open(self.full_path())?.dimensions();
        let _ = self.dimensions.set(dim);
        Ok(dim)
    }

    pub fn open(&self) -> Result<DynamicImage, image::ImageError> {
        image::open(self.full_path())
    }

    fn srcset(&self, path_to_root: &str, format: ImageFormat) -> String {
        Self::SIZES
            .iter()
            .map(|s| format!("{} {s}w", self.processed_web_url(path_to_root, *s, format)))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Asset for Img {
    fn key(&self) -> String {
        // Eindeutige Identität: Filesystem-Pfad
        self.full_path().to_string_lossy().into_owned()
    }

    fn process(
        self: Arc<Self>,
        prefix: &Path,
        overwrite: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let image = self.open()?;

        // Zielverzeichnis einmal anlegen
        let base_dir = prefix.join(&self.path).with_extension("");

        std::fs::create_dir_all(&base_dir)?;
        if overwrite || needs_copy(&self.full_path(), &self.copy_path(prefix)).unwrap_or(true) {
            std::fs::copy(self.full_path(), self.copy_path(prefix))?;
        }

        for format in Self::FORMATS {
            Self::SIZES
                .par_iter()
                .map(|size| (size, self.processed_fs_path(prefix, *size, format)))
                .filter(|(_, path)| overwrite || !path.exists())
                .map(|(size, path)| {
                    let resized = image.resize(*size as u32, u32::MAX, FilterType::Lanczos3);
                    (path, resized)
                })
                .map(|(path, img)| {
                    let img = match format {
                        ImageFormat::Jpeg => DynamicImage::ImageRgb8(img.to_rgb8()), // Alpha wegwerfen
                        ImageFormat::WebP => DynamicImage::ImageRgba8(img.to_rgba8()), // RGBA16 → RGBA8
                        _ => img,
                    };
                    (path, img)
                })
                .for_each(|(path, image)| {
                    let res = image.save_with_format(&path, format);
                    match res {
                        Ok(_) => println!("📁🖼️  Img Asset gespeichert: {}", path.display()),
                        Err(e) => {
                            println!("❌🖼️ Img Asset failed:\n\t{}\n\t{:?}", path.display(), e)
                        }
                    }
                });
        }
        // println!("Done {}", base_dir.display());
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ImgProps<'a> {
    pub path_to_root: &'a str,
    // TODO: tell the compile 1 (str1,str2) per Img::Sizes str1:media str2:width
    pub sizes: &'a str,
    pub eager: bool,
    pub id: Option<&'a str>,
    pub alt: Option<&'a str>,
    pub class: &'a [&'a str],
    pub style: Option<&'a str>,
    pub attrs: &'a [(&'a str, &'a str)],
    pub children: Option<Markup>,
}
