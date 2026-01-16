use std::{collections::HashMap, sync::OnceLock};

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    assets::img::{Img, ImgProps}, components::{
        self, Component,
        footer::footer,
        head::default_head,
        header::header,
        hyper_img::{self, Href, HyperMap, InsetPercent, MapNode},
        icon::{Icon, IconToMarkup},
        phone_border,
        project_table::{self, with_sub_heading},
        three_js_setup::import_map,
        tooltip,
    }, include_public, projects::ProjectMetadata, setup_language_loader
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
        title_img: 
        // link_public!(
        //     "assets/Design der Mensch Maschine Schnittstelle/WatchOut/title-img-flipp-bg.webp"
        // )
        Img::new("public", "assets/Design der Mensch Maschine Schnittstelle/WatchOut/title-img-flipp-bg.webp", "").unwrap()
        .into(),
        name: "WatchOut",
        description: loader.get("description").leak(),
        category: projects::Category::DMMS,
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
        (&*core_loader.get("module").leak(), html!(span.fit{(tooltip::markup(tooltip::MarkupProps {
            children: html!("DMMS"),
            content: html!("Design der Mensch Maschine Schnittstelle"),
                popup_align: tooltip::Align::Center,
                popup_justify: tooltip::Align::End,
                popup_begin_justify: tooltip::Align::Center,
                popup_begin_align: tooltip::Align::Center,
            }))})).into(),
        (&*core_loader.get("period").leak(), format!("{} 2024 - {} 2025",core_loader.get("October"),core_loader.get("February")).leak()).into(),
        (&*core_loader.get("team").leak(), PreEscaped(r#"<a class="link link-active underline"
                                        href="https://strangelifekid.github.io/Portfolio_Linda/" target="_blank"
                                        rel="noopener noreferrer">
                                        Linda Jakob</a>,
                                    Tim Ruland,
                                    <a class="link link-active underline"
                                        href="https://niiiicolaas.github.io/Nicolas-Weber-Portfolio/" target="_blank"
                                        rel="noopener noreferrer">
                                        Nicolas Weber</a>"#)).into(),
        (&*core_loader.get("tools").leak(), html!{ul."icon-row"{([
            Icon::Illustrator,
            Icon::Photoshop,
            Icon::Premiere,
            Icon::XD,
            Icon::Audacity,
            Icon::Blender,
            Icon::Git,
            Icon::GitHub
            ].to_markup(&page.path_to_root(lang)))}}).into(),
        (&*core_loader.get("university").leak(), "Technische Hochschule Ingolstadt").into(),
    ];

    let video_href = page.path_to_root(lang)
        + *link_public!("assets/Design der Mensch Maschine Schnittstelle/WatchOut/Video720_1.mp4");

    let meta_data = meta_data(lang);
    let dark_title_img = meta_data.title_img.dark();
    let light_title_img = meta_data.title_img.light();

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (import_map(page, lang))
                script type="module" src=(page.path_to_root(lang)+*link_public!("watchout.js")){}
                (hyper_img_script.render(&page.path_to_root(lang)))
                (default_head("WatchOut - Tim Ruland","Die ProduktKonzeptSeite von dem WatchOut-Gruppenprojekt", page, lang))
                (table_style.render(&page.path_to_root(lang)))
                (hyper_img_style.render(&page.path_to_root(lang)))
                (phone_border_style.render(&page.path_to_root(lang)))
                (tooltip::style().render(&page.path_to_root(lang)))
                style{
                    (PreEscaped(include_asset!("watchout.css")))
                }
            }

            body{
                (header(page, lang))
                main{
                    section #Hero{
                        picture #HeroImg{
                            (dark_title_img.render(ImgProps { path_to_root: &page.path_to_root(lang), eager: true, class: &["dark-only"], ..Default::default() }))
                            (light_title_img.render(ImgProps { path_to_root: &page.path_to_root(lang), eager: true, class: &["light-only"], ..Default::default() }))
                        }
                    }
                    (table_html(project_table::MarkupProps {
                        title: with_sub_heading("WatchOut App & Uhr","Design der Mensch-Maschine-Schnittstelle"),
                        graphic: html!{
                            video controls{
                                source src=(video_href) type="video/mp4";
                                a href=(video_href) type="video/mp4"{ "Download" }
                            }
                        }.into(),
                        rows,
                        text: (&*loader.get("content").leak()).into(),
                        long_text: true
                    }))
                    section.sect."accent-background" #FinishSect{
                        div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}
                        section.content #WatchSect{

                            div #WatchGrid{
                                h2.heading{ (loader.get("watch")) }
                                div #WatchText{
                                    p {
                                       (loader.get("watch-idea"))
                                    }
                                    p{
                                        (loader.get("watch-features"))
                                    }
                                }
                                canvas #WatchInfoCanvas{}
                            }
                        }
                        section.content #AppSect{
                            div #AppGrid{
                                h2.heading{ (loader.get("app")) }
                                div #AppText{
                                    p {
                                        (loader.get("app-general"))
                                    }
                                    p {
                                        (loader.get("app-call-history"))
                                    }
                                    p {
                                        (loader.get("app-map"))
                                    }
                                }
                                div #AppInfo{
                                    (phone_border_html(phone_border::MarkupProps{
                                        content: hyper_img_html(hyper_img::MarkupProps {
                                            map: watchout_hi_map(),
                                            path_to_root: page.path_to_root(lang),
                                            eager: false,
                                        }),
                                        path_to_root: page.path_to_root(lang),
                                        eager: false
                                    }))
                                }
                            }
                        }

                        div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                    }
                    div.sect{
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

    const HOME_STR: &str = "News";
    const HISTORY_STR: &str = "History";
    const MAP_STR: &str = "Map";
    const MAP_OVERLAY_HELEN_STR: &str = "Map_overlay_Helen";
    const MAP_OVERLAY_JOE_STR: &str = "Map_overlay_Joe";
    const MAP_OVERLAY_GUNTHER_STR: &str = "Map_overlay_Gunther";
    const SETTINGS_STR: &str = "Settings";
    const EVENT_OVERLAY_STR: &str = "Event_overlay";
    const EVENT_OVERLAY_PASSIVE_STR: &str = "Event_overlay_passive";
    const CALL_LIST_STR: &str = "CallList";

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

    let home_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Recent Events  – log.png", "").unwrap();
    let history_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/History  – 1 Log.png", "").unwrap();
    let maps_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Map – 1.png", "").unwrap();
    let map_overlay_helen_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Map Overlay – 1 – Event.png", "").unwrap();
    let map_overlay_joe_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Map Overlay – 2 – Joe.png", "").unwrap();
    let map_overlay_gunther_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Map Overlay – 2 – Gunther.png", "").unwrap();
    let settings_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Settings – 1.png", "").unwrap();
    let event_overlay_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Message Overlay – new.png", "").unwrap();
    let event_overlay_passive_links_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Message Overlay seen.png", "").unwrap();
    let call_list_links_img = Img::new("public", "./assets/Design der Mensch Maschine Schnittstelle/WatchOut/Watch out Exports/Call List Expanded.png", "").unwrap();

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
            img: home_img.clone(),
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
            img: history_img.clone(),
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
            img: maps_img.clone(),
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
            img: map_overlay_helen_img.clone(),
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
            img: map_overlay_joe_img.clone(),
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
            img: map_overlay_gunther_img.clone(),
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
            img: settings_img.clone(),
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
            img:event_overlay_img.clone(),
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
            img: event_overlay_passive_links_img.clone(),
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
            img: call_list_links_img.clone(),
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

    HyperMap(h_map, home_img.get_dimensions().unwrap())
}
