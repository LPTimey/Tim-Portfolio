use std::sync::OnceLock;

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    components::{
        carousel, codeblock, footer::footer, head::default_head, header::header, icon::{Icon, IconToMarkup}, img, page, project_table::{self, with_sub_heading}, tooltip, Component
    },
    include_public,
    projects::ProjectMetadata,
    setup_language_loader,
};

use super::super::*;

pub const TETRIS_C: &str = include_public!("assets/Tetris/Tetris_new/Tetris/src/GameState.cpp");
pub const TETRIS_H: &str = include_public!("assets/Tetris/Tetris_new/Tetris/src/GameState.hpp");

fn get_str_lines_range(input: &str, start: usize, end: usize) -> &str {
    let mut line_start = 0;
    let mut slice_start = 0;
    let mut slice_end = 0;

    for (idx, line) in input.lines().enumerate() {
        if idx == start {
            slice_start = line_start;
        }
        if idx == end {
            slice_end = line_start + line.len();
        }
        // advance line_start past this line + newline chars
        line_start += line.len();
        if let Some(ch) = input[line_start..].chars().next() {
            if ch == '\n' {
                line_start += 1;
            } else if ch == '\r' && input[line_start + 1..].starts_with('\n') {
                line_start += 2;
            }
        }
    }

    if end >= start {
        &input[slice_start..=slice_end]
    } else {
        ""
    }
}

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "tetris")
}

pub const MOD_PATH: &str = module_path!();
pub fn meta_data(lang: &LanguageIdentifier) -> ProjectMetadata {
    let loader = get_language_loader().select_languages(&[lang]);
    ProjectMetadata {
        page: Page::Tetris,
        title_img: link_public!("assets/Tetris/Title-img.webp").into(),
        name: "Tetris in Arduino & C",
        description: loader.get("description").leak(),
        category: projects::Category::Programmieren,
        favorite: true,
    }
}

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let loader = get_language_loader().select_languages(&[lang]);
    let core_loader = get_core_language_loader().select_languages(&[lang]);

    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();
    let Component {
        html: carousel_html,
        style: carousel_style,
        script: carousel_script
    } = carousel::component();
    let Component {
        html: codeblock_html,
        style: codeblock_style,
        script: codeblock_script
    } = codeblock::component();

    page::page(
        page.path_to_root(lang),
        html! {
            head{
                script type="module" src=(page.path_to_root(lang)+*carousel_script){}
                script type="module" src=(page.path_to_root(lang)+*codeblock_script){}
                (default_head("Tetris","//TODO: Add description",page, lang))
                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
                link rel="stylesheet" href=(page.path_to_root(lang)+*carousel_style);
                link rel="stylesheet" href=(page.path_to_root(lang)+*codeblock_style);
                link rel="stylesheet" href=(page.path_to_root(lang) + *tooltip::style() );
            }

            body{
                (header(page, lang))
                main{
                    section #Hero{
                        picture #HeroImg{(img::img (img::ImgProps {
                                pre_src: page.path_to_root(lang),
                                src: meta_data(lang).title_img.light(),
                                ..Default::default()
                            }))}
                    }
                    (table_html(project_table::MarkupProps {
                        // title: "Tetris auf dem Arduino?".into(),
                        title: with_sub_heading("Tetris auf dem Arduino?","Programmieren"),
                        graphic: html!{
                            picture{
                                (img::img (img::ImgProps {
                                    pre_src: page.path_to_root(lang),
                                    src: meta_data(lang).title_img.light(),
                                    ..Default::default()
                                }))
                            }
                        }.into(),
                        rows:&[
                            (&*core_loader.get("module").leak(), "TMMIP").into(),
                            (&*core_loader.get("period").leak(), format!("{} 2024 - {} 2025",core_loader.get("October"),core_loader.get("February")).leak()).into(),
                            (&*core_loader.get("tools").leak(), html!{ul."icon-row"{([
                                Icon::Fritzing,
                                Icon::VSCode,
                                Icon::Arduino,
                                Icon::Cpp,
                                Icon::Git,
                                Icon::GitHub
                                ].to_markup(&page.path_to_root(lang)))}}).into(),
                            (&*core_loader.get("university").leak(), "Technische Hochschule Ingolstadt").into(),
                        ],
                        text: loader.get("content").leak().into()
                    }))
                    section.sect.content{
                        h2.subhead{(loader.get("hardware"))" & "(loader.get("preparation"))}
                        p {(loader.get("hardware-prep"))}
                        ul{
                            li{"Arduino R3"}
                            li{"Arduino R4"}
                            li{(loader.get("breadboard"))}
                            li{(loader.get("resistors"))}
                            li{(loader.get("matrix"))}
                            li{(loader.get("connectors"))}
                        }
                        (img::img(img::ImgProps{pre_src:page.path_to_root(lang),src:link_public!("assets/Tetris/webp/Einzelteile.webp"),..Default::default()}))
                    }
                    section.sect."accent-background".content style="
                        --accent-bg-c: var(--black);
                        --bg-light: #e2d279ff;
                        --bg-normal: #F0D439;
                        --bg-dark: #F0BE3A;"{
                        div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}

                        h2.subhead{(loader.get("result"))}
                        p {(loader.get("result-coarse"))}
                        (carousel_html(carousel::MarkupProps { id: "ResultPhotos", pre_src: &page.path_to_root(lang), images:&[
                            link_public!("assets/Tetris/webp/Tetris_Steckplatine_small.webp"),
                            link_public!("assets/Tetris/webp/Buttons mit widerstand_small.webp"),
                            link_public!("assets/Tetris/webp/buttons mit + und gnd topview_small.webp"),
                            link_public!("assets/Tetris/webp/Button verbunden topview_small.webp"),
                        ] }))
                        h3."body-strong"{(loader.get("lessons-learned"))}
                        p{(loader.get("lessons-learned-text"))}
                        h3."body-strong"{(loader.get("follow-up"))}
                        p{(loader.get("follow-up-text"))}

                        div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                    }
                    section.sect.content{
                        h2.subhead{"Umsetzung"}
                        h3."body-strong"{"Code"}
                        p{
                            r#" Für den Hackathon, habe ich eine Tetris Bibliothek in C++ entwickelt und als Nachbereitung auf R3 erweitert. Diese basiert auf GameState welches alle wichtigen Daten einer runde speichert und Tick-Methoden bereitstellt um das spiel zu treiben. Es stellt auch eine Methode bereit, um das aktuelle Feld entweder als String oder Liste zu bekommen und es anzeigen zu können.

Die einzelnen Tetrominos sind in einem Enum als Indexe zu einem Array, welcher Postions-Matrizen der Tetrominos speichert. Um auch Position & Rotation zu speicher wird TetPos benutzt. 


Um die Nutzereingabe zu lesen werden 2 Typen exportiert: Buttons und Button. Button ist ein Enum welches alle Knöpfe auflistet und je einem Bit in einem Byte zuordnet, sodass alle möglichen Eingaben gleichzeitig und speichersparend verarbeitet werden können, da sie nun in einen Byte (Buttons) passen. Das ermöglicht schnelle Abfragen und kompakte Logik. Man kann sich dieses Flaggen-System vorstellen wie 8 boolesche Werte in einer Variable. Man kann diese Werte mit Bit shifts ( << ) und Bit-Oder ( | ) setzen und mit Bit shifts und Bit-Und ( & ) lesen. (Mehr Dazu)
"#
                        }
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: get_str_lines_range(TETRIS_H, 22, 31), prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: get_str_lines_range(TETRIS_H, 64, 75), prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: get_str_lines_range(TETRIS_H, 92, 98), prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: get_str_lines_range(TETRIS_H, 100, 135), prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: get_str_lines_range(TETRIS_C, 41, 54), prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: get_str_lines_range(TETRIS_H, 9, 20), prog_lang: "cpp" }))}
                        p{r#"
                        Die Erscheinungsraten der Tetrominos werden mit Hilfe eines Taschensystems generiert. Diese Tasche generiert alle Tetrominos und randomisiert ihre Order um zu garantieren, sodass es keine Folge "Ziehungen" gibt ein welcher eine art Tetromino öfter als 2 mal oder gar nicht vorkommt. 
                        "#}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: get_str_lines_range(TETRIS_H, 34, 48), prog_lang: "cpp" }))}
                        ("Graph")
                        h3."body-strong"{"Hardware"}
                        p{}
                    }
                    section.sect."accent-background".content style="
                        --accent-bg-c: var(--black);
                        --bg-light: #e2d279ff;
                        --bg-normal: #F0D439;
                        --bg-dark: #F0BE3A;"{
                        div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}

                        h2.subhead{"How to run"}
                        div{
                            h3.subhead{"Arduino"}
                            h4{(loader.get("req"))}
                            ul{
                                li{"Quellcode (Download)"}
                                li{"Arduino IDE"}
                                li{"Aufgelistete Hardware oder gleichwertige Komponenten"}
                            }
                            h4{(loader.get("step-by-step"))}
                            ul{
                                li{}
                                li{}
                                li{}
                                li{}
                                li{}
                            }
                        }
                        div{
                            h3.subhead{(loader.get("pc"))}
                            h4{(loader.get("req"))}
                            ul{
                                li{"Quellcode (Download)"}
                                li{"C/C++-Compiler (z.B: clang++)"}
                            }
                            h4{(loader.get("step-by-step"))}
                            ul{
                                li{}
                            }
                        }

                        // div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                    }
                }
                (footer(lang))
            }
        },
    )
}
