use maud::PreEscaped;

use crate::{
    components::{
        self, Component, footer::footer, head::default_head, header::header, project_card,
    },
    include_logo, placeholder_img,
};

use super::*;
pub const MOD_PATH: &str = module_path!();

pub const STYLE: &str = include_asset!("index.css");
pub const SCRIPT: &str = include_asset!("index.js");
pub const ICH: Link = link_public!("assets/Lebenslauf/schönes bild.JPG");

pub fn page(page: Page) -> maud::Markup {
    let Component {
        html,
        style: card_style,
        script: card_script,
    } = components::project_card::component();

    let skills = vec![
        (
            "Design",
            vec![
                include_logo!("adobe-illustrator-svgrepo-com.svg"),
                include_logo!("adobe-photoshop-svgrepo-com.svg"),
                include_logo!("adobe-premiere-svgrepo-com.svg"),
                include_logo!("adobe-xd-svgrepo-com.svg"),
                include_logo!("figma-svgrepo-com.svg"),
                include_logo!("penpot-svgrepo-com.svg"),
                include_logo!("blender-svgrepo-com.svg"),
            ],
        ),
        (
            "EDV",
            vec![
                include_logo!("office-1-logo-svgrepo-com.svg"),
                include_logo!("microsoft-windows-svgrepo-com.svg"),
                include_logo!("apple-nogb-svgrepo-com.svg"),
                include_logo!("linux-svgrepo-com.svg"),
                include_logo!("vs-code-svgrepo-com.svg"),
                include_logo!("neovim-mark@2x.svg"),
            ],
        ),
        (
            "Sprachen",
            vec![
                include_logo!("html-5-no-wordmark-svgrepo-com.svg"),
                include_logo!("CSS Logo.svg"),
                include_logo!("js-svgrepo-com.svg"),
                include_logo!("rust-svgrepo-com.svg"),
                include_logo!("java-svgrepo-com.svg"),
                include_logo!("cpp-svgrepo-com.svg"),
                include_logo!("python-svgrepo-com.svg"),
            ],
        ),
    ];

    components::page::page(html! {
        head{
            (default_head("Home","TODO: Add description", page.path_to_root()))
            style { (PreEscaped(STYLE)) }
            link rel="stylesheet" href=(page.path_to_root() + *card_style );
            script type="module" src=(page.path_to_root() + *card_script ){}
        }

        body{
            (header(page))
            main{
                section #Hero{
                    picture #HeroImg{img src=(*placeholder_img!(600,400)) alt="";}
                    div ."hero-content"{
                        h1."fmb-large"{
                            span."fs-large"."lh-tight"{ "Willkommen, hier" } br;
                            span.hero."lh-normal"{ "wo die Details scheinen" }
                        }
                        a class="btn accent-btn" href="#AboutMe" { "Entdecke mehr" }
                    }
                }

                section #AboutMe .content.sect."sect-large-start" {
                    h2.heading{ span."accent-text"{ "Hi! " } "Ich bin Tim." }
                    picture{img src=(page.path_to_root() + *ICH) alt="";}
                    p{
                        (PreEscaped(r#"
Mich faszinieren sowohl IT & Programmierung als auch Gestaltung & Design.
Aus diesem Zusammenspiel zwischen technischer Präzision und gestalterischem Denken ziehe ich die Motivation für meine Projekte.
Es ermöglicht mir, ansprechende und zugleich effiziente Lösungen zu entwickeln - immer mit einem strukturierten Vorgehen, großer Sorgfalt und einem ausgeprägten Blick für Details.
Derzeit studiere ich User Experience Design an der <a target="_blank" href="https://thi.de" class="link link-active underline">Technischen Hochschule Ingolstadt</a>.
In meinem Studium wie auch in meinem eigenen Schaffen lege ich großen Wert auf Verlässlichkeit, Teamarbeit und einen verantwortungsvollen Umgang mit sensiblen Daten.
Neben dem Studium musiziere ich, fahre gerne Rad und game, natürlich alles auch mit Freunden.

Ich freue mich, wenn du dir einen Eindruck von meiner Arbeit verschaffst. Bei Fragen oder Interesse an einer Zusammenarbeit, melde dich gerne!
                        "#))
                    }
                }

                section #Erfahrung .content.sect {
                    div #Werdegang {
                        h2{ "Werdegang" }
                        div .timeline {
                            div ."timeline-item" {
                                span ."timeline-date" {
                                    "2023 - Heute"
                                }
                                div ."timeline-content" {
                                    h3 { "Technische Hochschule Ingolstadt" }
                                    p { "UX Design Studium (B.A.)" }
                                }
                            }
                            div ."timeline-item" {
                                span ."timeline-date" {
                                    "2020 - 2022"
                                }
                                div ."timeline-content" {
                                    h3{ "FOS/BOS Scheyern" }
                                    p{"Technik-Zweig"}
                                }
                            }
                            div ."timeline-item" {
                                span ."timeline-date" {
                                    "2014 - 2020"
                                }
                                div ."timeline-content" {
                                    h3 { "Georg-Hipp Realschule" }
                                    p { "Mathematik-Zweig" }
                                }
                            }
                        }
                    }
                    div #Skills{
                        h2 { "Skills" }
                        div #SkillCategories {
                            @for (i,category) in skills.iter().enumerate(){
                                div .category{
                                    h3 { (category.0) }

                                    div {
                                        @for (j,svg) in category.1.iter().enumerate() {
                                            div ."skill-icon"{
                                                (PreEscaped(
                                                    svg
                                                    // stop svg's from changing each others styles, by having duplicate ids
                                                    .replace("url(#", &format!("url(#{i}{j}"))
                                                    .replace("id=\"", &format!("id=\"{i}{j}"))
                                                ))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                section #Projects .sect{
                    div .content{
                        (html(project_card::MarkupProps{title:"WatchOut",description:"Eine Uhr und eine App um Menschen mit Demenz und deren Familie zu helfen ihr Leben sorgloser zu leben.",img:"",theme:"DMMS"}))
                        (html(project_card::MarkupProps{title:"Drucker Touchscreen",description:"",img:"",theme:"Screendesign"}))
                        (html(project_card::MarkupProps{title:"Themen & Stile",description:"",img:"",theme:"Screendesign"}))
                        (html(project_card::MarkupProps{title:"Tetris in Arduino & C",description:"",img:"",theme:"Programmieren"}))
                    }
                }
            }
            (footer())
            script{(PreEscaped(SCRIPT))}
        }
    })
}
