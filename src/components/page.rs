use maud::{html, PreEscaped};

pub fn page(content: maud::Markup)->maud::Markup{
    html!{
        (PreEscaped("<!DOCTYPE html>"))

        html lang="de"{
            (content)
        }
    }
}