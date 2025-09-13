use std::sync::OnceLock;

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    TabIndex,
    components::{
        Component, carousel, codeblock,
        footer::footer,
        head::default_head,
        header::header,
        icon::{Icon, IconToMarkup},
        img, mermaid, page,
        project_table::{self, with_sub_heading},
        tooltip::{self, Align},
    },
    include_public,
    projects::ProjectMetadata,
    setup_language_loader,
};

use super::super::*;

// const TETRIS_C: &str = include_public!("assets/Tetris/Tetris_new/Tetris/src/GameState.cpp");
const TETRIS_H: &str = include_public!("assets/Tetris/Tetris_new/Tetris/src/GameState.hpp");

const TET_STATE: &str = "\
class GameState {
    struct M {
        uint8_t field[Width * Height];
        Option&lt;TetPos&gt; current;
        Tetromino nexts[3];
        TetrominoBag bag;
        size_t score = 0;
    } m;

    GameState() = delete;
    GameState(M&& members);

public:
    static GameState create();
    bool is_game_over();
    bool game_tick();
    bool player_tick(Buttons buttons);
    auto field_with_floating ->
    (uint8_t (&arr)[Width * Height]);
    int32_t to_string
    (char (&str)[Width*Height*2 + 1]);
};";
const TET_BUTTONS: &str = "\
typedef uint8_t Buttons;
enum class Button: Buttons {
    TurnLeft  = 1, // 00000001
    TurnRight = 2, // 00000010
    Up    =  4,    // 00000100
    Down  =  8,    // 00001000
    Left  = 16,    // 00010000
    Right = 32,    // 00100000
    Swap  = 64,    // 01000000
    Max_Button,    // size + 1
};";
const TET_POS_ROT: &str = "\
struct Vec2 {
    int8_t x, y;
};

enum class Rotation: uint8_t {
    None   = 0,       //   0°
    CounterClock = 1, //  90°
    Mirror = 2,       // 180°
    Clock  = 3,       // 270°
    Max_Rotation // für Modulo
};";
const TET_TET_POS: &str = "\
struct TetPos {
    Tetromino cur;
    Vec2 cur_pos;
    Rotation rot;

    auto get_indexes()
    -> ptrdiff_t (&)[4];
};";
const TET_TETROMINO: &str = "\
// Index für Array
enum class Tetromino: uint8_t {
    L   = 0, // -> Tetrominos[0]
    R_L = 1, // -> Tetrominos[1]
    I   = 2, // -> Tetrominos[2]
    Z   = 3, // -> Tetrominos[3]
    R_Z = 4, // -> Tetrominos[4]
    B   = 5, // -> Tetrominos[5]
    T   = 6, // -> Tetrominos[6]
    NrOfTetrominos, // Wie .len
};";
const TET_TET_VEC_1: &str = "\
// ... Die anderen Tetrominos ...
constexpr Vec2 T[] =
    { {0, 0}, {0, -1}, {-1, 0}, {1, 0} };
constexpr Vec2* Tetrominos[] =
    { L, R_L, I, Z, R_Z, B, T };";
const _TET_TET_VEC_2: &str = "\
// ... Die anderen Tetrominos ...
constexpr Vec2 T[] = { {0, 0}, {0, -1}, {-1, 0}, {1, 0} };
constexpr Vec2* Tetrominos[] = { L, R_L, I, Z, R_Z, B, T };";
const _TET_BAG: &str = "\
using Tetromino as T;
class TetrominoBag {
    Tetromino bag
        [(uint8_t)T::NrOfTetrominos];
    uint8_t index = 0;
    uint32_t seed = 12345;
    void set_next_batch();
public:
    TetrominoBag();
    TetrominoBag(uint32_t seed);
    void set_seed(uint32_t seed);
    Tetromino next();
};";

fn get_tet_bag_diagram(_lang: &LanguageIdentifier, horizontal: bool) -> String {
    let res = format!(
        r#"
flowchart {}
Use[["bag.next()"]]
CallSetNextBatch["set_next_batch()"]
FillBag["Befülle Bag <br>mit allen 7 Tetrominos"]
Shuffle["Shuffle Bag <br>(Fisher-Yates, seed-basiert)"]
ResetIndex["Setze index = 0"]
NextCall[/"next() aufgerufen"/]
SaveTetromino["speichere this->bag[index]<br>und erhöhe index"]
ReturnTetromino["gespeicherten Tetromino zurückgeben"]
CheckIndex{{"index &lt; 7?"}}
RefillBag[/"Alle Tetrominos verwendet<br>set_next_batch() aufrufen"/]

    Use --> NextCall
    CallSetNextBatch --> FillBag
    FillBag --> Shuffle
    Shuffle --> ResetIndex
    ResetIndex --> ReturnTetromino
    NextCall --> SaveTetromino
    SaveTetromino --> CheckIndex
    CheckIndex -- Ja --> ReturnTetromino
    CheckIndex -- Nein --> RefillBag
    RefillBag --> CallSetNextBatch
    ReturnTetromino --> Use
"#,
        if horizontal { "LR" } else { "TD" }
    );
    // println!("{}",res);
    res
}

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
    let mut _tab_index = TabIndex::default();
    let meta_data = meta_data(lang);
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
        script: carousel_script,
    } = carousel::component();
    let Component {
        html: codeblock_html,
        style: codeblock_style,
        script: codeblock_script,
    } = codeblock::component();
    let Component {
        html: mermaid_html,
        script: mermaid_script,
        ..
    } = mermaid::component();

    page::page(
        page.path_to_root(lang),
        html! {
            head{
                script type="module" src=(page.path_to_root(lang)+*carousel_script){}
                script type="module" src=(page.path_to_root(lang)+*codeblock_script){}
                script type="module" src=(page.path_to_root(lang)+*mermaid_script){}
                style{(PreEscaped(include_asset!("tetris.css")))}
                (default_head("Tetris",meta_data.description,page, lang))
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
                                src: meta_data.title_img.light(),
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
                                    src: meta_data.title_img.light(),
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
                        h2.subhead."mb-medium"{(loader.get("hardware"))" & "(loader.get("preparation"))}
                        p {(loader.get("hardware-prep"))}
                        ul{
                            li{"Arduino R3"}
                            li{"Arduino R4"}
                            li{(loader.get("breadboard"))}
                            li{(loader.get("resistors"))}
                            li{(loader.get("matrix"))}
                            li{(loader.get("connectors"))}
                        }
                        picture #EinzelteilePic{
                            div.marker #UnoR4Marker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped("What is going on".to_owned()),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                            div.marker #UnoR3Marker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped("What is going on".to_owned()),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                            div.marker #MatrixMarker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped("What is going on".to_owned()),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                            div.marker #ResistorsMarker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped("What is going on".to_owned()),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                            div.marker #ExtrasMarker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped("What is going on".to_owned()),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                            div.marker #BreadboardMarker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped("What is going on".to_owned()),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                            (img::img(img::ImgProps{pre_src:page.path_to_root(lang),src:link_public!("assets/Tetris/webp/Einzelteile.webp"),..Default::default()}))
                        }
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
                        div."mb-medium"{
                            h3.subhead{(loader.get("lessons-learned"))}
                            p{(loader.get("lessons-learned-text"))}
                        }
                        h3.subhead{(loader.get("follow-up"))}
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
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: TET_STATE, prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: TET_BUTTONS, prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: TET_POS_ROT, prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: TET_TET_POS, prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: TET_TET_VEC_1, prog_lang: "cpp" }))}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: TET_TETROMINO, prog_lang: "cpp" }))}
                        p{r#"
                        Die Erscheinungsraten der Tetrominos werden mit Hilfe eines Taschensystems generiert. Diese Tasche generiert alle Tetrominos und randomisiert ihre Order um zu garantieren, sodass es keine Folge "Ziehungen" gibt ein welcher eine art Tetromino öfter als 2 mal oder gar nicht vorkommt. 
                        "#}
                        pre{(codeblock_html(codeblock::MarkupProps { id: "", data: get_str_lines_range(TETRIS_H, 34, 48), prog_lang: "cpp" }))}

                        (mermaid_html(mermaid::MarkupProps { name: "TetrinoDiagram", defs: &[("horizontal",&get_tet_bag_diagram(lang, true)),("vertical",&get_tet_bag_diagram(lang, false))] }))
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
