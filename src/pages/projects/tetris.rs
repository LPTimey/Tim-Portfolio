use std::sync::OnceLock;

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    components::{
        carousel, footer::footer, head::default_head, header::header, icon::{Icon, IconToMarkup}, img, page, project_table::{self, with_sub_heading}, tooltip, Component
    },
    include_public,
    projects::ProjectMetadata,
    setup_language_loader,
};

use super::super::*;

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
        ..
    } = carousel::component();

    page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head("Tetris","//TODO: Add description",page, lang))
                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
                link rel="stylesheet" href=(page.path_to_root(lang)+*carousel_style);
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

Die einzelnen Tetrinos sind in einem Enum als Indexe zu einem Array, welcher Postions-Matrizen der Tetrinos speichert. Um auch Position & Rotation zu speicher wird TetPos benutzt. 


Um die Nutzereingabe zu lesen werden 2 Typen exportiert: Buttons und Button. Button ist ein Enum welches alle Knöpfe auflistet und je einem Bit in einem Byte zuordnet, sodass alle möglichen Eingaben gleichzeitig und speichersparend verarbeitet werden können, da sie nun in einen Byte (Buttons) passen. Das ermöglicht schnelle Abfragen und kompakte Logik. Man kann sich dieses Flaggen-System vorstellen wie 8 boolesche Werte in einer Variable. Man kann diese Werte mit Bit shifts ( << ) und Bit-Oder ( | ) setzen und mit Bit shifts und Bit-Und ( & ) lesen. (Mehr Dazu)
"#
                        }
                        ("6x Code")
                        p{r#"
                        Die Erscheinungsraten der Tetrinos werden mit Hilfe eines Taschensystems generiert. Diese Tasche generiert alle Tetrinos und randomisiert ihre Order um zu garantieren, sodass es keine Folge "Ziehungen" gibt ein welcher eine art Tetrino öfter als 2 mal oder gar nicht vorkommt. 
                        "#}
                        ("Code")
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
