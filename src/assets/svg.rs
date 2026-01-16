use std::{io, path::PathBuf, sync::Arc};

use maud::{Markup, html};

use crate::{
    assets::{Asset, register_seen_or_get, register_used},
    canonicalize_web_path,
};

#[derive(Debug)]
pub struct Svg {
    /// Filesystem-Prefix, z. B. ./public
    pub prefix: PathBuf,

    /// Web-relativer Pfad, z. B. assets/img/profile.jpg
    pub path: PathBuf,

    /// Fallback alt (kann beim Render überschrieben werden)
    pub alt: String,
}
impl Svg {
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
        });

        // Registriere oder hole vorhandenes Asset
        let registered: Arc<dyn Asset> = register_seen_or_get(candidate.clone() as Arc<dyn Asset>);

        Ok(Arc::downcast::<Self>(registered).unwrap_or(candidate))
    }
    fn full_path(&self) -> PathBuf {
        self.prefix.join(&self.path)
    }
    fn web_path(&self, path_to_root: &str) -> String {
        path_to_root.to_string() + &self.path.to_string_lossy()
    }
    fn processed_fs_path(&self, prefix: impl Into<PathBuf>) -> PathBuf {
        prefix.into().join(&self.path)
    }
    pub fn render(self: Arc<Self>, props: SvgProps) -> Markup {
        register_used(&(self.clone() as Arc<dyn Asset>));
        html! {
            img
                id=(props.id.unwrap_or(""))
                src=(canonicalize_web_path(&self.web_path(props.path_to_root)))
                alt=(props.alt.unwrap_or(&self.alt))
                class=(props.class.join(" "))
                style=(props.style.unwrap_or(""))
                decoding="async"
                loading=(if props.eager {"eager"} else{"lazy"})
                draggable="false"
                width="40"
                height="40";
        }
    }
}
impl Asset for Svg {
    fn key(&self) -> String {
        self.full_path().to_string_lossy().into_owned()
    }

    fn process(
        self: std::sync::Arc<Self>,
        prefix: &std::path::Path,
        overwrite: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.full_path().exists() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Datei existiert nicht: {}", self.full_path().display()),
            )));
        }
        let full_path = self.processed_fs_path(prefix);
        if !overwrite && full_path.exists() {
            return Ok(());
        }

        std::fs::copy(self.full_path(), full_path)?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct SvgProps<'a> {
    pub path_to_root: &'a str,
    pub eager: bool,
    pub id: Option<&'a str>,
    pub alt: Option<&'a str>,
    pub class: &'a [&'a str],
    pub style: Option<&'a str>,
}
