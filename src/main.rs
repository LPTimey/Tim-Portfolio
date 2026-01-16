use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use clap::Parser;
use rayon::prelude::*;
use strum::VariantArray;
use website::Page;
use website::SUPPORTED_LANGS;
use website::used_assets;

fn main() {
    // Page::VARIANTS.iter().for_each(|page| {
    //     println!("{}: {}", page.to_path().display(), page.to_markup().0);
    //     println!();
    // });
    let cli = Cli::parse();

    let _ = cli.build();
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets the output Directory
    #[arg(short, long, value_name = "DIR", default_value = "dist")]
    out: PathBuf,
    #[arg(long, default_value = "false")]
    overwrite: bool,
}
impl Cli {
    pub fn build(&self) -> io::Result<()> {
        self.build_pages()?;
        // Should be useless now with Assets
        // self.copy_public_assets("public")?;
        used_assets().into_par_iter().for_each(|asset| {
            let _res = asset.process(&self.out, self.overwrite);
            // let _ = dbg!(res);
        });
        Ok(())
    }

    fn build_pages(&self) -> io::Result<()> {
        SUPPORTED_LANGS.par_iter().for_each(|lang| {
            Page::VARIANTS.into_par_iter().for_each(|page| {
                let relative_path = page.to_href(lang);
                let full_path = self.out.join(relative_path);

                if let Some(parent) = full_path.parent() {
                    match fs::create_dir_all(parent) {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("❌  Fehler: {}", parent.display());
                            return;
                        }
                    };
                }

                let markup = page.to_markup(lang).0;
                let needs_copy = match fs::read_to_string(&full_path) {
                    Ok(dest_contents) => dest_contents != markup,
                    Err(_) => true, // Ziel existiert nicht => muss kopiert werden
                };
                if !needs_copy {
                    println!("✔️  Seite vorhanden: {}", full_path.display());
                    return;
                }
                match fs::write(&full_path, markup) {
                    Ok(_) => println!("✔️  Seite geschrieben: {}", full_path.display()),
                    Err(_) => eprintln!("❌  Fehler: {}", full_path.display()),
                };
            })
        });
        Ok(())
    }

    fn copy_public_assets<P: AsRef<Path>>(&self, public_dir: P) -> io::Result<()> {
        fn copy_recursive(from: &Path, to: &Path) -> io::Result<()> {
            if from.is_dir() {
                fs::create_dir_all(to)?;
                for entry in fs::read_dir(from)? {
                    let entry = entry?;
                    let file_type = entry.file_type()?;
                    let from_path = entry.path();
                    let to_path = to.join(entry.file_name());

                    if file_type.is_dir() {
                        copy_recursive(&from_path, &to_path)?;
                    } else if file_type.is_file() {
                        let needs_copy = match fs::metadata(&to_path) {
                            Ok(dest_metadata) => {
                                let src_metadata = fs::metadata(&from_path)?;
                                let src_modified = src_metadata.modified()?;
                                let dest_modified = dest_metadata.modified()?;
                                let src_size = src_metadata.len();
                                let dest_size = dest_metadata.len();

                                src_modified > dest_modified || src_size != dest_size
                            }
                            Err(_) => true, // Ziel existiert nicht => muss kopiert werden
                        };

                        if needs_copy {
                            fs::create_dir_all(to_path.parent().unwrap())?;
                            fs::copy(&from_path, &to_path)?;
                            println!("📁 Asset kopiert: {}", to_path.display());
                        }
                    }
                }
            }
            Ok(())
        }

        let from = public_dir.as_ref();
        let to = &self.out;
        copy_recursive(from, to)
    }
}
