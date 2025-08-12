use std::{collections::HashMap, sync::OnceLock};

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    components::{
        self, footer::footer, head::default_head, header::header, hyper_img::{self, Href, HyperMap, InsetPercent, MapNode}, img, phone_border, project_table::{self, with_sub_heading}, three_js_setup::import_map, Component
    },
    include_public,
    projects::ProjectMetadata, setup_language_loader,
};

use super::super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "watchout")
}

pub const MOD_PATH: &str = module_path!();
pub fn meta_data(lang: &LanguageIdentifier) -> ProjectMetadata {
    let loader = get_language_loader().select_languages(&[lang]);
    ProjectMetadata {
        page: Page::Watchout,
        title_img: link_public!(
            "assets/Design der Mensch Maschine Schnittstelle/WatchOut/title-img-flipp-bg.webp"
        )
        .into(),
        name: "WatchOut",
        description: loader.get("description").leak(),
        category: projects::Category::DMMS,
        favorite: true,
    }
}

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let loader = get_language_loader().select_languages(&[lang]);

    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();

    let Component {
        html: phone_border_html,
        style: phone_border_style,
        ..
    } = phone_border::component();

    let Component {
        html: hyper_img_html,
        style: hyper_img_style,
        script: hyper_img_script,
    } = hyper_img::component();

    let rows = &[
        ("Studienmodul", "DMMS").into(),
        ("Zeitraum", "Oktober 2024 - Februar 2025").into(),
        ("Team", PreEscaped(r#"<a class="link link-active underline"
                                        href="https://strangelifekid.github.io/Portfolio_Linda/" target="_blank"
                                        rel="noopener noreferrer">
                                        Linda Jakob</a>,
                                    Tim Ruland,
                                    <a class="link link-active underline"
                                        href="https://niiiicolaas.github.io/Nicolas-Weber-Portfolio/" target="_blank"
                                        rel="noopener noreferrer">
                                        Nicolas Weber</a>"#)).into(),
        ("Tools", "Illustrator, Photoshop, Premiere Pro, XD, Audacity, Blender, git, GitHub").into(),
        ("Hochschule", "Technische Hochschule Ingolstadt").into(),
    ];

    let video_href = page.path_to_root(lang)
        + *link_public!("assets/Design der Mensch Maschine Schnittstelle/WatchOut/Video720_1.mp4");

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (import_map(page, lang))
                script type="module" src=(page.path_to_root(lang)+*link_public!("watchout.js")){}
                script type="module" src=(page.path_to_root(lang)+*hyper_img_script){}
                (default_head("WatchOut - Tim Ruland","Die ProduktKonzeptSeite von dem WatchOut-Gruppenprojekt",page.path_to_root(lang), lang))
                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
                link rel="stylesheet" href=(page.path_to_root(lang)+*hyper_img_style);
                link rel="stylesheet" href=(page.path_to_root(lang)+*phone_border_style);
                style{
                    (PreEscaped(include_asset!("watchout.css")))
                }
            }

            body{
                (header(page, lang))
                main{
                    section #Hero{
                        picture #HeroImg{(img::img (page.path_to_root(lang),meta_data(lang).title_img.light(),"",None,&[],None))}
                        div ."hero-content"{
                            h1."mb-large"{
                                span."fs-large"."lh-tight"{ "Willkommen, hier" } br;
                                span.hero."lh-normal"."fw-gigantic"{ "wo die Details scheinen" }
                            }
                            a draggable="false" .btn."accent-btn".shadow href="#AboutMe" { "Entdecke mehr" }
                        }
                    }
                    (table_html(project_table::MarkupProps {
                        // title: html!{
                        //     span."fw-normal"."ui-small"."lh-tight".block{
                        //         "Design der Mensch-Maschine-Schnittstelle"
                        //     }
                        //     "WatchOut: Motivation & Generelles"
                        // }.into(),
                        title: with_sub_heading("WatchOut App & Uhr","Design der Mensch-Maschine-Schnittstelle"),
                        graphic: html!{
                            video controls{
                                source src=(video_href) type="video/mp4";
                                a href=(video_href) type="video/mp4"{ "Download" }
                            }
                        }.into(),
                        rows,
                        text: (&*loader.get("content").leak()).into()
                    }))
                    section.sect."accent-background" #FinishSect{
                        div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}
                        section.content #WatchSect{

                            div #WatchGrid{
                                h2.heading{ "Uhr" }
                                div #WatchText{
                                    p {
                                        "Da Demenz oft die vertrauten Gewohnheiten und Erinnerungen der Betroffenen am längsten
                                        bewahrt, wurde die Uhr im klassischen, analogen Design gestaltet. "
                                    }
                                    p{
                                        "Die Gestaltung zielt darauf ab, der Uhr eine vertraute Bedeutung zu verleihen.
                                    Um den Bedürfnissen der oft älteren Zielgruppe gerecht zu werden,
                                    sind sowohl die Ziffern als auch die Zeiger gut lesbar und groß.
                                    Zudem ist die Uhr ergonomisch abgerundet und aus einem weichen Material
                                    gefertigt, um Verletzungen vorzubeugen. "
                                    }
                                    p {
                                        "Die Uhr sendet GPS-Daten, verfügt über eine aktive Fallerkennung und eine
                                    Notruffunktion mit den 2 Knöpfen. Zusätzlich behält sie ihre Funktion als gewöhnliche Analoguhr
                                    mit Krone bei. "
                                    }
                                }
                                canvas #WatchInfoCanvas{}
                            }
                        }
                        section.content #AppSect{
                            div #AppGrid{
                                h2.heading{ "Begleitapp" }
                                div #AppText{
                                    p {
                                        "Die mit der Uhr verbundene App wurde nach dem Prinzip der Schlichtheit gestaltet. "
                                    }
                                    p {
                                        "Die Startseite bietet einen Überblick über die letzten Ereignisse.
                                    Um die Übersichtlichkeit zu gewährleisten, sind diese Ereignisse zusammengefasst,
                                    nach Zeit sortiert und farblich nach Schweregrad kategorisiert.
                                    Außerdem ermöglicht die Übersicht, die uhrtragende Person, gespeicherte Kontakte
                                    oder die Notfallstelle direkt anzurufen. "
                                    }
                                    p {
                                        "In der Historie werden neben den aktuellen Ereignissen auch vergangene Ereignisse
                                        angezeigt, die detaillierte Einblicke in frühere Aktivitäten und Notfälle bieten. "
                                    }
                                    p {
                                        "Zusätzlich enthält die App eine Karte, mit der die Position der erkrankten Person
                                        sowie die anderer Angehöriger überprüft werden kann. "
                                    }
                                }
                                div #AppInfo{
                                    (phone_border_html(phone_border::MarkupProps{content: hyper_img_html(hyper_img::MarkupProps { map: watchout_hi_map(), path_to_root: page.path_to_root(lang) }), path_to_root: page.path_to_root(lang)}))
                                }
                            }
                        }

                        div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                    }
                    section.sect{
                        ""
                    }
                }
                (footer(lang))
            }
        },
    )
}

pub fn watchout_hi_map() -> HyperMap {
    let mut h_map = HashMap::new();

    const HOME_STR: &'static str = "News";
    const HISTORY_STR: &'static str = "History";
    const MAP_STR: &'static str = "Map";
    const MAP_OVERLAY_HELEN_STR: &'static str = "Map_overlay_Helen";
    const MAP_OVERLAY_JOE_STR: &'static str = "Map_overlay_Joe";
    const MAP_OVERLAY_GUNTHER_STR: &'static str = "Map_overlay_Gunther";
    const SETTINGS_STR: &'static str = "Settings";
    const EVENT_OVERLAY_STR: &'static str = "Event_overlay";
    const EVENT_OVERLAY_PASSIVE_STR: &'static str = "Event_overlay_passive";
    const CALL_LIST_STR: &'static str = "CallList";

    let navbar = [
        (
            InsetPercent::new(90, 75, 2, 10),
            Href::Specific(HOME_STR.to_string()),
        ),
        (
            InsetPercent::new(90, 53, 2, 31),
            Href::Specific(HISTORY_STR.to_string()),
        ),
        (
            InsetPercent::new(90, 30, 2, 52),
            Href::Specific(MAP_STR.to_string()),
        ),
        (
            InsetPercent::new(90, 10, 2, 75),
            Href::Specific(SETTINGS_STR.to_string()),
        ),
    ];

    let mut home_links = vec![
        (
            InsetPercent::new(40, 10, 50, 10),
            Href::Specific(EVENT_OVERLAY_STR.to_string()),
        ),
        (
            InsetPercent::new(81, 7, 13, 7),
            Href::Specific(CALL_LIST_STR.to_string()),
        ),
    ];
    home_links.extend_from_slice(&navbar);
    let home = (
        HOME_STR.to_string(),
        MapNode {
            buttons: home_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Recent Events  – log.png"
            ),
            alpha: false,
            default: true,
        },
    );

    let mut history_links = vec![
        (
            InsetPercent::new(81, 7, 12, 7),
            Href::Specific(CALL_LIST_STR.to_string()),
        ),
        (
            InsetPercent::new(33, 8, 56, 8),
            Href::Specific(EVENT_OVERLAY_STR.to_string()),
        ),
        (
            InsetPercent::new(69, 8, 21, 8),
            Href::Specific(EVENT_OVERLAY_PASSIVE_STR.to_string()),
        ),
    ];
    history_links.extend_from_slice(&navbar);
    let history = (
        HISTORY_STR.to_string(),
        MapNode {
            buttons: history_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/History  – 1 Log.png"
            ),
            alpha: false,
            default: false,
        },
    );

    let mut map_links = vec![
        (
            InsetPercent::new(81, 7, 12, 7),
            Href::Specific(CALL_LIST_STR.to_string()),
        ),
        (
            InsetPercent::new(32, 30, 54, 45),
            Href::Specific(MAP_OVERLAY_HELEN_STR.to_string()),
        ),
        (
            InsetPercent::new(50, 15, 40, 70),
            Href::Specific(MAP_OVERLAY_JOE_STR.to_string()),
        ),
        (
            InsetPercent::new(62, 62, 28, 23),
            Href::Specific(MAP_OVERLAY_GUNTHER_STR.to_string()),
        ),
    ];
    map_links.extend_from_slice(&navbar);
    let map = (
        MAP_STR.to_string(),
        MapNode {
            buttons: map_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Map – 1.png"
            ),
            alpha: false,
            default: false,
        },
    );

    let mut map_overlay_helen_links = vec![
        (
            InsetPercent::new(39, 12, 51, 12),
            Href::Specific(HOME_STR.to_string()),
        ),
        (
            InsetPercent::new(81, 7, 12, 7),
            Href::Specific(CALL_LIST_STR.to_string()),
        ),
        // Top
        (InsetPercent::new(0, 0, 72, 0), Href::Back),
        // Left
        (InsetPercent::new(0, 93, 21, 0), Href::Back),
        // Right
        (InsetPercent::new(0, 0, 21, 93), Href::Back),
        // Bottom
        (InsetPercent::new(60, 0, 21, 0), Href::Back),
    ];
    map_overlay_helen_links.extend_from_slice(&navbar);
    let map_overlay_helen = (
        MAP_OVERLAY_HELEN_STR.to_string(),
        MapNode {
            buttons: map_overlay_helen_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Map Overlay – 1 – Event.png"
            ),
            alpha: true,
            default: false,
        },
    );

    let mut map_overlay_joe_links = vec![
        (
            InsetPercent::new(81, 7, 12, 7),
            Href::Specific(CALL_LIST_STR.to_string()),
        ),
        // Top
        (InsetPercent::new(0, 0, 72, 0), Href::Back),
        // Left
        (InsetPercent::new(0, 93, 21, 0), Href::Back),
        // Right
        (InsetPercent::new(0, 0, 21, 93), Href::Back),
        // Bottom
        (InsetPercent::new(60, 0, 21, 0), Href::Back),
    ];
    map_overlay_joe_links.extend_from_slice(&navbar);
    let map_overlay_joe = (
        MAP_OVERLAY_JOE_STR.to_string(),
        MapNode {
            buttons: map_overlay_joe_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Map Overlay – 2 – Joe.png"
            ),
            alpha: true,
            default: false,
        },
    );

    let mut map_overlay_gunther_links = vec![
        (
            InsetPercent::new(81, 7, 12, 7),
            Href::Specific(CALL_LIST_STR.to_string()),
        ),
        // Top
        (InsetPercent::new(0, 0, 72, 0), Href::Back),
        // Left
        (InsetPercent::new(0, 93, 21, 0), Href::Back),
        // Right
        (InsetPercent::new(0, 0, 21, 93), Href::Back),
        // Bottom
        (InsetPercent::new(60, 0, 21, 0), Href::Back),
    ];
    map_overlay_gunther_links.extend_from_slice(&navbar);
    let map_overlay_gunther = (
        MAP_OVERLAY_GUNTHER_STR.to_string(),
        MapNode {
            buttons: map_overlay_gunther_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Map Overlay – 2 – Gunther.png"
            ),
            alpha: true,
            default: false,
        },
    );

    let mut settings_links = vec![];
    settings_links.extend_from_slice(&navbar);
    let settings = (
        SETTINGS_STR.to_string(),
        MapNode {
            buttons: settings_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Settings – 1.png"
            ),
            alpha: false,
            default: false,
        },
    );

    let mut event_overlay_links = vec![
        (
            InsetPercent::new(38, 15, 44, 15),
            Href::Specific(MAP_STR.to_string()),
        ),
        // Top
        (InsetPercent::new(0, 0, 76, 0), Href::Back),
        // Left
        (InsetPercent::new(0, 93, 21, 0), Href::Back),
        // Right
        (InsetPercent::new(0, 0, 21, 93), Href::Back),
        // Bottom
        (InsetPercent::new(66, 0, 21, 0), Href::Back),
    ];
    event_overlay_links.extend_from_slice(&navbar);
    let event_overlay = (
        EVENT_OVERLAY_STR.to_string(),
        MapNode {
            buttons: event_overlay_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Message Overlay – new.png"
            ),
            alpha: true,
            default: false,
        },
    );

    let mut event_overlay_passive_links = vec![
        (
            InsetPercent::new(38, 15, 44, 15),
            Href::Specific(MAP_STR.to_string()),
        ),
        // Top
        (InsetPercent::new(0, 0, 76, 0), Href::Back),
        // Left
        (InsetPercent::new(0, 93, 21, 0), Href::Back),
        // Right
        (InsetPercent::new(0, 0, 21, 93), Href::Back),
        // Bottom
        (InsetPercent::new(66, 0, 21, 0), Href::Back),
    ];
    event_overlay_passive_links.extend_from_slice(&navbar);
    let event_overlay_passive = (
        EVENT_OVERLAY_PASSIVE_STR.to_string(),
        MapNode {
            buttons: event_overlay_passive_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Message Overlay seen.png"
            ),
            alpha: true,
            default: false,
        },
    );

    let mut call_list_links = vec![(InsetPercent::new(0, 0, 53, 0), Href::Back)];
    call_list_links.extend_from_slice(&navbar);
    let call_list = (
        CALL_LIST_STR.to_string(),
        MapNode {
            buttons: call_list_links,
            img: link_public!(
                "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Call List Expanded.png"
            ),
            alpha: true,
            default: false,
        },
    );

    h_map.extend([
        home,
        history,
        map,
        map_overlay_helen,
        map_overlay_joe,
        map_overlay_gunther,
        settings,
        event_overlay,
        event_overlay_passive,
        call_list,
    ]);

    HyperMap(h_map)
}
