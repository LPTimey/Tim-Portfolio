use std::{
    io,
    path::PathBuf,
    sync::Arc,
};

use maud::{Markup, html};

use crate::{
    assets::{Asset, register_seen_or_get, register_used},
    canonicalize_web_path, needs_copy,
};

#[derive(Debug)]
pub struct StyleSheet {
    /// Filesystem-Prefix, z. B. ./public
    pub prefix: PathBuf,

    /// Web-relativer Pfad, z. B. assets/img/profile.jpg
    pub path: PathBuf,
}
impl StyleSheet {
    pub fn new(
        prefix: impl Into<PathBuf>,
        path: impl Into<PathBuf>,
    ) -> Result<Arc<StyleSheet>, io::Error> {
        let candidate = Arc::new(Self {
            prefix: prefix.into(),
            path: path.into(),
        });

        if !candidate.full_path().is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Datei existiert nicht: {}", candidate.full_path().display()),
            ));
        }
        let registered: Arc<dyn Asset> = register_seen_or_get(candidate.clone() as Arc<dyn Asset>);

        Ok(Arc::downcast::<StyleSheet>(registered).unwrap_or(candidate))
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
    pub fn render(self: Arc<Self>, path_to_root: &str) -> Markup {
        register_used(&(self.clone() as Arc<dyn Asset>));
        html! {
            link href=(canonicalize_web_path(&self.web_path(path_to_root))) rel="stylesheet";
        }
    }
}
impl Asset for StyleSheet {
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
                format!("❌  Original Datei existiert nicht: {}", self.full_path().display()),
            )));
        }
        let full_path = self.processed_fs_path(prefix);
        if !(overwrite || needs_copy(&full_path, &self.full_path())?) {
            return Ok(());
        }

        if let Some(path) = full_path.parent() {
            match std::fs::create_dir_all(&path) {
                Ok(val) => {
                    println!("📁📂  Dirs erstellt: {}", path.display());
                    Ok(val)
                }
                Err(err) => {
                    eprintln!("❌📂  Dirs failed:\n\t{}\n\t{:?}", path.display(), err);
                    Err(err)
                }
            }?;
        }

        match std::fs::copy(self.full_path(), &full_path) {
            Ok(val) => {
                println!("📁  StyleSheet Asset gespeichert: {}", full_path.display());
                Ok(val)
            },
            Err(err) => {
                eprintln!("❌  StyleSheet Asset failed:\n\t{}\n\t{:?}", full_path.display(), err);
                Err(err)
            },
        }?;

        Ok(())
    }
}
