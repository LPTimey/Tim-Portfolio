use maud::PreEscaped;

use crate::{
    components::{
        Component,
        footer::footer,
        head::default_head,
        header::header,
        hyper_img::{self, Href, HyperMap, InsetPercent, MapNode},
        img, page,
        project_table::{self, with_sub_heading},
    },
    projects::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Anpassung des Hochschuldrucker UIs für bessere Les- und Bedienbarkeit ohne den Verlust von Features."#;
const CONTENT: PreEscaped<&'static str> = PreEscaped(
    r#"
    Drucker sind berüchtigt für frustrierende Bedienung. 
    Veraltete Benutzeroberflächen, verschachtelte Menüs und unklare Abläufe sorgen oft dafür, 
    dass sie nicht so funktionieren, wie es Nutzer erwarten.

    Im Rahmen eines Studienprojekts habe ich das veraltete, wenig benutzerfreundliche UI der 
    hochschuleigenen Drucker neu gestaltet. Das Redesign optimiert insbesondere die Touch-Bedienung 
    durch größere Buttons, klarere Nutzerführung und eine reduzierte Komplexität. Der Login-Bildschirm 
    ist vereinfacht, zentrale Funktionen wie Sprache und Tintenfüllstand sind direkt zugänglich, und 
    der gesamte Druckprozess wird durch eine klar strukturierte Schritt-für-Schritt-Navigation begleitet.
"#,
);

pub const MOD_PATH: &str = module_path!();
pub fn meta_data(lang: &LanguageIdentifier) -> ProjectMetadata {
    ProjectMetadata {
        page: Page::Printer,
        title_img: link_public!(
            (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path())
                + "assets/Screendesign/Drucker/title-img-zoomed.webp")
                .leak()
        )
        .into(),
        name: "Drucker Touchscreen",
        description: DESCRIPTION,
        category: projects::Category::Screendesign,
        favorite: true,
    }
}

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();

    let Component {
        html: hy_img_html,
        style: hy_img_style,
        script: hy_img_script,
    } = hyper_img::component();

    page::page(
        page.path_to_root(lang),
        html! {
            head{
                script type="module" src=(page.path_to_root(lang)+*hy_img_script) {}
                (default_head("Drucker",DESCRIPTION,page.path_to_root(lang),lang))
                link rel="stylesheet" href=(page.path_to_root(lang)+*hy_img_style);
                link rel="stylesheet" href=(page.path_to_root(lang)+*table_style);
                style{(PreEscaped(include_asset!("printer.css")))}
            }

            body{
                (header(page,lang))
                main{
                    section #Hero{
                        picture #HeroImg{(img::img (page.path_to_root(lang),link_public!("assets/Screendesign/Drucker/title-img.webp"),"",None,&[],None))}
                    }
                    (table_html(project_table::MarkupProps {
                        // title: "Drucker: Motivation & Generelles".into(),
                        title: with_sub_heading("Drucker Re-Design","Screendesign"),
                        // graphic: (page.path_to_root(lang),meta_data(lang).title_img.light()).into(),
                        graphic: html!{(hy_img_html(hyper_img::MarkupProps { map: hyper_map(), path_to_root: page.path_to_root(lang) }))}.into(),
                        rows:&[
                            ("Studienmodul", "Projekt Gestaltung II").into(),
                            ("Zeitraum", "März 2024 - Juli 2024").into(),
                            ("Tools", "Illustrator, XD, git, GitHub").into(),
                            ("Hochschule", "Technische Hochschule Ingolstadt").into(),
                        ],
                        text: CONTENT.into()
                    }))
                    section.sect.content #Login{
                        picture{(img::img(
                            page.path_to_root(lang),
                            link_public!("assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_05.png"),
                            "",
                            None,
                            &[],
                            None)
                        )}
                        div {
                            h2.heading{ "Login" }
                            p{
                                r#"
                                Drucker sind leider oft dafür bekannt, nicht so zu funktionieren, wie es die Nutzer erwarten. Ein großer Teil dieses Problems liegt in der meist unzureichenden Nutzererfahrung (UX).

                                Oft beginnt dies schon bei einem überladenen Login-Bildschirm. Der neue Login-Bildschirm wurde jedoch vereinfacht und ist mit großen Buttons und gut lesbarem Text für kleine Touchscreens optimiert.

                                Die wichtigsten Einstellungen des Drucker-UIs, wie Sprache und Füllstand, sind sofort sichtbar und können einfach angepasst werden. Weitere Einstellungen sind ebenfalls schnell und problemlos erreichbar.
                                "#
                            }
                        }
                    }
                    section.sect.content #PrintPath{
                        div{
                            h2.heading{ "Druckvorgang" }
                            p {
                                "Nach dem Login mit Passwort oder NFC-Karte wird
                                der Nutzername zusammen mit dem Kontostand in
                                der Anzeigeschablone angezeigt."
                            }
                            ul.list{
                                li{
                                    "Die Druckoptionen sind groß, kontrastreich und mit
                                    klaren Icons versehen, um die Bedienung zu erleichtern."
                                }
                                li{
                                    "Das Abmelden erfolgt über das Nutzermenü, das über
                                    den Nutzernamen-Knopf erreichbar ist."
                                }
                                li{
                                    "Der Nutzer wird von einem klar strukturierten Leitfaden
                                    durch den gesamten Druckprozess geführt,
                                    sodass er stets weiß, wo er ist und welcher Schritt als
                                    nächstes kommt."
                                }
                                li{
                                    "Der aktuelle Schritt wird hervorgehoben, während
                                    noch nicht abgeschlossene Schritte ausgegraut bleiben,
                                    um Verwirrung zu vermeiden."
                                }
                            }
                        }
                        picture{(img::img(
                            page.path_to_root(lang),
                            link_public!("assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_07.png"),
                            "",
                            None,
                            &[],
                            None)
                        )}
                        picture{(img::img(
                            page.path_to_root(lang),
                            link_public!("assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_08.png"),
                            "",
                            None,
                            &[],
                            None)
                        )}
                        picture{(img::img(
                            page.path_to_root(lang),
                            link_public!("assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_09.png"),
                            "",
                            None,
                            &[],
                            None)
                        )}
                        picture{(img::img(
                            page.path_to_root(lang),
                            link_public!("assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_13.png"),
                            "",
                            None,
                            &[],
                            None)
                        )}
                    }
                }
                (footer(lang))
            }
        },
    )
}

pub fn hyper_map() -> HyperMap {
    let mut hyper_map = HyperMap::default();
    let map = &mut hyper_map.0;

    const LOGIN_STR: &'static str = "LoginPage";
    const LOGIN_CARD_STR: &'static str = "Login_CardPage";
    const HOME_STR: &'static str = "HomePage";
    const SETTINGS_STR: &'static str = "SettingsPage";
    const LANG_STR: &'static str = "LanguagePage";
    const COLOR_STR: &'static str = "ColorPage";
    const USER_STR: &'static str = "UserPage";
    const AUSWAHL_STR: &'static str = "AuswahlPage";
    const AUSWAHL_ALLES_STR: &'static str = "AuswahlAllesPage";
    const EINSTELLEN_STR: &'static str = "EinstellenPage";
    const DRUCKEN_STR: &'static str = "DruckenPage";
    const AFTER_STR: &'static str = "AfterPage";

    let _nav = [
        (
            InsetPercent::new(0, 83, 85, 0),
            Href::Specific(SETTINGS_STR.to_string()),
        ),
        (
            InsetPercent::new(85, 83, 0, 0),
            Href::Specific(LANG_STR.to_string()),
        ),
        (
            InsetPercent::new(85, 0, 0, 83),
            Href::Specific(COLOR_STR.to_string()),
        ),
    ];
    let _nav_user = (
        InsetPercent::new(0, 0, 85, 83),
        Href::Specific(USER_STR.to_string()),
    );

    let login_links = vec![
        (
            InsetPercent::new(17, 39, 74, 43),
            Href::Specific(LOGIN_CARD_STR.to_string()),
        ),
        (
            InsetPercent::new(67, 32, 23, 32),
            Href::Specific(HOME_STR.to_string()),
        ),
    ];
    // login_links.extend_from_slice(&nav);
    let login = (
        LOGIN_STR.to_string(),
        MapNode {
            buttons: login_links,
            img: link_public!(
                "assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_05.png"
            ),
            alpha: false,
            default: true,
        },
    );

    let login_card_links = vec![(
        InsetPercent::new(17, 56, 74, 26),
        Href::Specific(LOGIN_STR.to_string()),
    )];
    // login_card_links.extend_from_slice(&nav);
    let login_card = (
        LOGIN_CARD_STR.to_string(),
        MapNode {
            buttons: login_card_links,
            img: link_public!(
                "assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_06.png"
            ),
            alpha: false,
            default: false,
        },
    );

    let home_links = vec![
        (
            InsetPercent::new(16, 50, 51, 22),
            Href::Specific(AUSWAHL_STR.to_string()),
        ),
        (
            InsetPercent::new(16, 21, 51, 51),
            Href::Specific(EINSTELLEN_STR.to_string()),
        ),
        (
            InsetPercent::new(51, 50, 16, 22),
            Href::Specific(EINSTELLEN_STR.to_string()),
        ),
    ];
    // home_links.extend_from_slice(&nav);
    // home_links.push(nav_user.clone());
    let home = (
        HOME_STR.to_string(),
        MapNode {
            buttons: home_links,
            img: link_public!(
                "assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_07.png"
            ),
            alpha: false,
            default: false,
        },
    );

    let auswahl_links = vec![
        (
            InsetPercent::new(74, 20, 15, 60),
            Href::Specific(EINSTELLEN_STR.to_string()),
        ),
        (InsetPercent::new(74, 59, 15, 21), Href::Back),
        (
            InsetPercent::new(30, 23, 63, 72),
            Href::Specific(AUSWAHL_ALLES_STR.to_string()),
        ),
    ];
    // auswahl_links.extend_from_slice(&nav);
    // auswahl_links.push(nav_user.clone());
    let auswahl = (
        AUSWAHL_STR.to_string(),
        MapNode {
            buttons: auswahl_links,
            img: link_public!(
                "assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_08.png"
            ),
            alpha: false,
            default: false,
        },
    );

    let auswahl_alles_links = vec![
        (
            InsetPercent::new(74, 20, 15, 60),
            Href::Specific(EINSTELLEN_STR.to_string()),
        ),
        (
            InsetPercent::new(74, 59, 15, 21),
            Href::Specific(HOME_STR.to_string()),
        ),
        (InsetPercent::new(30, 23, 63, 72), Href::Back),
    ];
    // auswahl_alles_links.extend_from_slice(&nav);
    // auswahl_alles_links.push(nav_user.clone());
    let auswahl_alles = (
        AUSWAHL_ALLES_STR.to_string(),
        MapNode {
            buttons: auswahl_alles_links,
            img: link_public!(
                "assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_08 - alles.webp"
            ),
            alpha: false,
            default: false,
        },
    );

    let einstellen_links = vec![
        (
            InsetPercent::new(74, 20, 15, 60),
            Href::Specific(DRUCKEN_STR.to_string()),
        ),
        (InsetPercent::new(74, 59, 15, 21), Href::Back),
        (
            InsetPercent::new(18, 58, 73, 23),
            Href::Specific(AUSWAHL_STR.to_string()),
        ),
    ];
    // einstellen_links.extend_from_slice(&nav);
    // einstellen_links.push(nav_user.clone());
    let einstellen = (
        EINSTELLEN_STR.to_string(),
        MapNode {
            buttons: einstellen_links,
            img: link_public!(
                "assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_09.png"
            ),
            alpha: false,
            default: false,
        },
    );

    let drucken_links = vec![
        (
            InsetPercent::new(74, 20, 15, 60),
            Href::Specific(AFTER_STR.to_string()),
        ),
        (InsetPercent::new(74, 59, 15, 21), Href::Back),
        (
            InsetPercent::new(18, 58, 73, 23),
            Href::Specific(AUSWAHL_STR.to_string()),
        ),
        (
            InsetPercent::new(18, 40, 73, 43),
            Href::Specific(EINSTELLEN_STR.to_string()),
        ),
    ];
    // drucken_links.extend_from_slice(&nav);
    // drucken_links.push(nav_user.clone());
    let drucken = (
        DRUCKEN_STR.to_string(),
        MapNode {
            buttons: drucken_links,
            img: link_public!(
                "assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_13.png"
            ),
            alpha: false,
            default: false,
        },
    );

    let after_links = vec![
        (
            InsetPercent::new(33, 52, 33, 7),
            Href::Specific(LOGIN_STR.to_string()),
        ),
        (
            InsetPercent::new(33, 9, 33, 53),
            Href::Specific(HOME_STR.to_string()),
        ),
    ];
    // after_links.extend_from_slice(&nav);
    // after_links.push(nav_user.clone());
    let after = (
        AFTER_STR.to_string(),
        MapNode {
            buttons: after_links,
            img: link_public!(
                "assets/Screendesign/Drucker/Tim_Ruland_Drucker_Screendesign_Seite_21.png"
            ),
            alpha: false,
            default: false,
        },
    );

    map.extend([
        login,
        login_card,
        home,
        auswahl,
        auswahl_alles,
        einstellen,
        drucken,
        after,
    ]);

    hyper_map
}
