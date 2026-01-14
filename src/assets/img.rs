use std::{io, path::PathBuf, sync::Arc};

use image::{GenericImageView, ImageFormat};
use maud::{Markup, PreEscaped, html};

use super::{Asset, register_or_get, register_used};

#[derive(Debug)]
pub struct Img {
    /// Filesystem-Prefix, z. B. ./public
    pub prefix: PathBuf,

    /// Web-relativer Pfad, z. B. assets/img/profile.jpg
    pub path: PathBuf,

    pub alt: String,
}

impl Img {
    pub const SIZES: [u16; 7] = [360, 412, 768, 1024, 1260, 1920, 2560];
    pub const FORMATS: [ImageFormat; 3] = [ImageFormat::Avif, ImageFormat::WebP, ImageFormat::Jpeg];

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
            prefix: prefix.into(),
            path: path.into(),
            alt: alt.into(),
        });

        // Registriere oder hole vorhandenes Asset
        let registered: Arc<dyn Asset> = register_or_get(candidate.clone() as Arc<dyn Asset>);

        // Versuche Downcast zurück zu Arc<Img>
        Ok(if let Ok(img) = Arc::downcast::<Img>(registered) {
            img
        } else {
            candidate
        })
    }

    pub fn render(self: Arc<Self>, props: ImgProps<'_>) -> Markup {
        // automatische Registrierung beim Rendern
        register_used(&(self.clone() as Arc<dyn Asset>));

        PreEscaped(format!(
            r#"<picture id='{id}' class='{class}' style='{style}' {attrs}>{html}</picture>"#,
            id = props.id.unwrap_or_default(),
            class = (props.class.join(" ")),
            style = (props.style.unwrap_or_default()),
            attrs = props
                .attrs
                .into_iter()
                .map(|(name, value)| format!(r#"{name}="{value}""#))
                .collect::<Vec<_>>()
                .join(" "),
            html = html! {
            // TODO: Reactivate after impl process
            // source
            //     type=(Self::FORMATS[0].to_mime_type())
            //     srcset=(self.srcset(Self::FORMATS[0]));

            // TODO: Reactivate after impl process
            // source
            //     type=(Self::FORMATS[1].to_mime_type())
            //     srcset=(self.srcset(Self::FORMATS[1]));

            img
                src=(self.web_path(props.path_to_root))
                // TODO: Reactivate after impl process
                // srcset=(self.srcset(Self::FORMATS[2]))
                alt=(self.alt)
                loading="lazy"
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

    /// Web-Pfad (HTML-safe, immer `/`)
    pub fn web_path(&self, path_to_root: &str) -> String {
        format!(
            "{path_to_root}/{}",
            self.path.to_string_lossy().replace('\\', "/")
        )
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

    pub fn get_dimensions(&self)->(u32,u32){
        image::open(self.full_path()).unwrap().dimensions()
    }

    fn srcset(&self, format: ImageFormat) -> String {
        let base = self.web_base_path();
        let ext = format.extensions_str()[0];

        Self::SIZES
            .iter()
            .map(|s| format!("{base}-{s}.{ext} {s}w"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Asset for Img {
    fn key(&self) -> String {
        // Eindeutige Identität: Filesystem-Pfad
        self.full_path().to_string_lossy().into_owned()
    }

    fn process(self: Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct ImgProps<'a> {
    pub path_to_root: &'a str,
    pub eager: bool,
    pub id: Option<&'a str>,
    pub alt: &'a str,
    pub class: &'a [&'a str],
    pub style: Option<&'a str>,
    pub attrs: &'a [(&'a str, &'a str)],
    pub children: Option<Markup>
}
