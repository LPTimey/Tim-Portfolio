use std::{io, path::PathBuf, sync::Arc};

use maud::{Markup, html};

use crate::{
    assets::{Asset, register_seen_or_get, register_used},
    canonicalize_web_path,
};

#[derive(Debug)]
pub struct Video {
    /// Filesystem-Prefix, z. B. ./public
    pub prefix: PathBuf,

    /// Web-relativer Pfad, z. B. assets/img/profile.jpg
    pub path: PathBuf,
}
impl Video {
    pub fn new(
        prefix: impl Into<PathBuf>,
        path: impl Into<PathBuf>,
    ) -> Result<Arc<Video>, io::Error> {
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

        Ok(Arc::downcast::<Video>(registered).unwrap_or(candidate))
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
            video controls{
                source src=(canonicalize_web_path(&self.web_path(path_to_root))) type="video/mp4";
                a href=(canonicalize_web_path(&self.web_path(path_to_root))) type="video/mp4" {
                    "Download"
                }
            }
        }
    }
}
impl Asset for Video {
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
                format!(
                    "❌  Original Datei existiert nicht: {}",
                    self.full_path().display()
                ),
            )));
        }
        let full_path = self.processed_fs_path(prefix);
        if !overwrite && full_path.exists() {
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
                println!("📁  Video Asset gespeichert: {}", full_path.display());
                Ok(val)
            }
            Err(err) => {
                eprintln!(
                    "❌  Video Asset failed:\n\t{}\n\t{:?}",
                    full_path.display(),
                    err
                );
                Err(err)
            }
        }?;

        Ok(())
    }
}
