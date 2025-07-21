use maud::{html, PreEscaped};

pub fn page(path_to_root:String, content: maud::Markup)->maud::Markup{
    html!{
        (PreEscaped("<!DOCTYPE html>"))

        html lang="de" style=(format!("--path-to-root:'{path_to_root}'")){
            (content)
        }
    }
}