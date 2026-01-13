use std::sync::OnceLock;

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    TabIndex, assets::img::{Img, ImgProps}, components::{
        Component, carousel, codeblock,
        footer::footer,
        head::default_head,
        header::header,
        icon::{Icon, IconToMarkup},
        mermaid, page,
        project_table::{self, with_sub_heading},
        tooltip::{self, Align},
    }, include_public, projects::ProjectMetadata, setup_language_loader
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
const TET_TET_VEC_SHORT: &str = "\
// ... Die anderen Tetrominos ...
constexpr Vec2 T[] =
    { {0, 0}, {0, -1}, {-1, 0}, {1, 0} };
constexpr Vec2* Tetrominos[] =
    { L, R_L, I, Z, R_Z, B, T };";
const TET_TET_VEC_LONG: &str = "\
// ... Die anderen Tetrominos ...
constexpr Vec2 T[] = { {0, 0}, {0, -1}, {-1, 0}, {1, 0} };
constexpr Vec2* Tetrominos[] = { L, R_L, I, Z, R_Z, B, T };";
const TET_BAG_SHORT: &str = "\
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
const TET_BAG_LONG: &str = "\
using Tetromino as T;
class TetrominoBag {
    Tetromino bag[(uint8_t)T::NrOfTetrominos];
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
        title_img: 
        // link_public!("assets/Tetris/Title-img.webp")
        Img::new("public","assets/Tetris/Title-img.webp","").unwrap()
        .into(),
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

    let dark_title_img = meta_data.title_img.dark();
    let light_title_img = meta_data.title_img.light();

    let einzelteile_img = Img::new("public", "assets/Tetris/webp/Einzelteile.webp", "").unwrap();
    let carousel_img_1= Img::new("public","assets/Tetris/webp/Tetris_Steckplatine_small.webp","").unwrap();
    let carousel_img_2 = Img::new("public","assets/Tetris/webp/Buttons mit widerstand_small.webp","").unwrap();
    let carousel_img_3 = Img::new("public","assets/Tetris/webp/buttons mit + und gnd topview_small.webp","").unwrap();
    let carousel_img_4 = Img::new("public","assets/Tetris/webp/Button verbunden topview_small.webp","").unwrap();

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
                        picture #HeroImg{
                            (dark_title_img.clone().render(ImgProps { path_to_root: &page.path_to_root(lang), eager: true, class: &["dark-only"], ..Default::default() }))
                            (light_title_img.clone().render(ImgProps { path_to_root: &page.path_to_root(lang), eager: true, class: &["light-only"], ..Default::default() }))
                        }
                    }
                    (table_html(project_table::MarkupProps {
                        // title: "Tetris auf dem Arduino?".into(),
                        title: with_sub_heading("Tetris auf dem Arduino?","Programmieren"),
                        graphic: html!{
                            (dark_title_img.render(ImgProps { path_to_root: &page.path_to_root(lang), eager: true, class: &["dark-only"], ..Default::default() }))
                            (light_title_img.render(ImgProps { path_to_root: &page.path_to_root(lang), eager: true, class: &["light-only"], ..Default::default() }))
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
                        text: loader.get("content").leak().into(),
                        long_text: false
                    }))
                    section.sect.content{
                        div #PrepGrid {
                            div.grow {
                                h2.heading."mb-medium"{(loader.get("hardware"))" & "(loader.get("preparation"))}
                                p {(PreEscaped(loader.get("hardware-prep")))}
                                ul.list {
                                    // TODO: Translate
                                    // TODO: href
                                    li {"Arduino UNO R4 WiFi (" a{} ")"}
                                    li {(loader.get("UNO-R3"))}
                                    li {
                                        (loader.get("breadboard"))
                                        " "
                                        (loader.get("small"))
                                        " ("
                                        (loader.get("optional"))
                                        ")"
                                    }
                                    li {(loader.get("resistors"))" (6x 1k\u{2126})"}
                                    li {(loader.get("cables"))", "(loader.get("buttons"))" & "(loader.get("big")) " " (loader.get("breadboard"))}
                                    li {(loader.get("wave-screen"))}
                                }
                            }
                            picture #EinzelteilePic{
                                div.marker #UnoR4Marker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped("Arduino R4".to_owned()),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                                div.marker #UnoR3Marker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped("Arduino R3".to_owned()),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                                div.marker #MatrixMarker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped(loader.get("matrix")),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                                div.marker #ResistorsMarker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped(loader.get("resistors")),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                                div.marker #ExtrasMarker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped(loader.get("connectors")),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                                div.marker #BreadboardMarker{ (tooltip::markup(tooltip::MarkupProps{children:html!{ div."accent-point"{} }, content: PreEscaped(loader.get("breadboard")),popup_align:Align::Center,popup_justify:Align::Center,popup_begin_align:Align::Center,popup_begin_justify:Align::Center })) }
                                (einzelteile_img.render(ImgProps{path_to_root:&page.path_to_root(lang),..Default::default()}))
                            }
                        }
                    }
                    section.sect."accent-background".content style="
                        --accent-bg-c: var(--black);
                        --bg-light: #e2d279ff;
                        --bg-normal: #F0D439;
                        --bg-dark: #F0BE3A;"{
                        div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}

                        h2.heading{(loader.get("result"))}
                        p {(loader.get("result-coarse"))}
                        (carousel_html(carousel::MarkupProps { id: "ResultPhotos", pre_src: &page.path_to_root(lang), eager:false, images:&[
                            carousel_img_1,
                            carousel_img_2,
                            carousel_img_3,
                            carousel_img_4,
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
                        h2.heading{"Umsetzung"}
                        h3.subhead{"Code"}
                        // TODO: Layout
                        div #Bento {
                            p #StateText{
                                (loader.get("game-state-text"))
                            }
                            pre #TET_STATE_Code {(codeblock_html(codeblock::MarkupProps { id: "", data: TET_STATE, prog_lang: "cpp" }))}
                            p #InputText {
                                (loader.get("input-text")) "("(loader.get("more-on"))")"
                            }
                            pre #TET_BUTTONS_Code {(codeblock_html(codeblock::MarkupProps { id: "", data: TET_BUTTONS, prog_lang: "cpp" }))}
                            pre #TET_POS_ROT_Code {(codeblock_html(codeblock::MarkupProps { id: "", data: TET_POS_ROT, prog_lang: "cpp" }))}
                            p #TetArrayText {
                                (loader.get("tet-array-text"))
                            }
                            pre #TET_TET_POS_Code {(codeblock_html(codeblock::MarkupProps { id: "", data: TET_TET_POS, prog_lang: "cpp" }))}
                            pre #TET_TET_VEC_1_Code {(codeblock_html(codeblock::MarkupProps { id: "", data: TET_TET_VEC_SHORT, prog_lang: "cpp" }))}
                            pre #TET_TETROMINO_Code {(codeblock_html(codeblock::MarkupProps { id: "", data: TET_TETROMINO, prog_lang: "cpp" }))}
                        }
                        div #Bag {
                            p #BagText {(loader.get("tet-bag-text"))}
                            pre #TET_BAG_CODE {(codeblock_html(codeblock::MarkupProps { id: "", data: TET_BAG_SHORT, prog_lang: "cpp" }))}

                            (mermaid_html(mermaid::MarkupProps { name: "TetrominoDiagram", defs: &[("horizontal",&get_tet_bag_diagram(lang, true)),("vertical",&get_tet_bag_diagram(lang, false))] }))
                        }
                        h3.subhead{"Hardware"}
                        // TODO: Content
                        p{}
                    }
                    section.sect."accent-background".content #RunSect style="
                        --accent-bg-c: var(--black);
                        --bg-light: #e2d279ff;
                        --bg-normal: #F0D439;
                        --bg-dark: #F0BE3A;"{
                        div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}

                        h2.heading{(loader.get("how-run"))}
                        div #RunCards {
                            div.card.shadow."no-hover" {
                                h3.subhead{"Arduino"}
                                div.group{
                                    h4."body-strong"{(loader.get("requirements"))":"}
                                    ul{
                                        li{
                                            (loader.get("src"))
                                            " ("
                                            // TODO: href
                                            a.link.underline href=""{
                                                (loader.get("download"))
                                            }
                                            ")"
                                        }
                                        li{"Arduino IDE"}
                                        li{(loader.get("listed-hardware"))}
                                    }
                                }
                                div.group{
                                    h4."body-strong"{(loader.get("step-by-step"))":"}
                                    ul{
                                        li{(loader.get("hardware-build"))}
                                        li{(loader.get("de-zip"))}
                                        li{(loader.get("cd"))}
                                        li{(loader.get("open-ide"))}
                                        li{(loader.get("build-ino"))}
                                    }
                                }
                            }
                            div.card.shadow."no-hover" {
                                h3.subhead{(loader.get("pc"))}
                                div.group{
                                    h4."body-strong"{(loader.get("requirements"))":"}
                                    ul{
                                        li{
                                            (loader.get("src"))
                                            " ("
                                            // TODO: href
                                            a.link.underline href=""{
                                                (loader.get("download"))
                                            }
                                            ")"
                                        }
                                        li{(loader.get("compiler"))}
                                    }
                                }
                                div.group{
                                    h4."body-strong"{(loader.get("step-by-step"))":"}
                                    ul{
                                        li{(loader.get("de-zip"))}
                                        li{(loader.get("cd"))}
                                        li{
                                            (loader.get("build-nob"))
                                            ul{
                                                li {
                                                    (loader.get("unix-like"))
                                                    ": "
                                                    (codeblock_html(codeblock::MarkupProps { id: "", data: "clang -o nob nob.c", prog_lang: "bash" }))
                                                }
                                                li {
                                                    "Windows"
                                                    ": "
                                                    (codeblock_html(codeblock::MarkupProps { id: "", data: "clang -o nob.exe nob.c", prog_lang: "bash" }))
                                                }
                                            }
                                        }
                                        li{
                                            "nob ausführen"
                                            ul{
                                                li {
                                                    (loader.get("unix-like"))
                                                    ": "
                                                    (codeblock_html(codeblock::MarkupProps { id: "", data: "./nob", prog_lang: "bash" }))
                                                }
                                                li {
                                                    "Windows"
                                                    ": "(codeblock_html(codeblock::MarkupProps { id: "", data: ".\\nob.exe", prog_lang: "bash" }))
                                                }
                                            }
                                        }
                                    }
                                }
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
