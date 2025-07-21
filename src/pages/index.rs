use std::time::Duration;

use maud::PreEscaped;

use crate::{
    components::{
        self, footer::footer, head::default_head, header::header, project_card, scrolling_img, Component
    },
    include_logo, include_public,
};

use super::*;
pub const MOD_PATH: &str = module_path!();

pub const STYLE: &str = include_asset!("index.css");
pub const SCRIPT: &str = include_asset!("index.js");
pub const ICH: Link = link_public!("assets/Lebenslauf/schönes bild klein bg@0,33x.jpg");

pub fn page(page: Page) -> maud::Markup {
    let Component {
        html: project_card_html,
        style: card_style,
        ..
    } = project_card::component();
    let Component {
        html: scroll_img_html,
        style: scroll_img_style,
        ..
    } = scrolling_img::component();

    let skills = [
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
            link rel="stylesheet" defer href=(page.path_to_root() + *card_style );
            // script type="module" src=(page.path_to_root() + *card_script ){}
            link rel="stylesheet" href=(page.path_to_root() + *scroll_img_style );
        }

        body{
            (header(page))
            main{
                section #Hero{
                    // picture #HeroImg{img draggable="false" src=(*placeholder_img!(600,400)) alt="";}
                    div #HeroImg{(scroll_img_html(scrolling_img::MarkupProps { img: Link((page.path_to_root()+*link_public!("assets/Title-img.webp")).leak()), rows: 3, columns: 3, duration: Duration::from_secs(50) }))}
                    div ."hero-content"{
                        h1."mb-large"{
                            span."fs-large"."lh-tight"{ "Willkommen, hier" } br;
                            span.hero."lh-normal"."fw-gigantic"{ "wo die Details scheinen" }
                        }
                        a draggable="false" .btn."accent-btn".shadow href="#AboutMe" { span{"Entdecke mehr"} }
                    }
                }


                section #AboutMe .sect."sect-large-start"."sect-small-end" {
                    div.content {
                        h2.heading{ span."accent-text"{ "Hi! " } "Ich bin Tim." }
                        picture{img draggable="false" src=(page.path_to_root() + *ICH) alt="";}
                        p{
                            (PreEscaped(r#"
Mich faszinieren sowohl IT & Programmierung als auch Gestaltung & Design.
Aus diesem Zusammenspiel zwischen technischer Präzision und gestalterischem Denken ziehe ich die Motivation für meine Projekte.
Es ermöglicht mir, ansprechende und zugleich effiziente Lösungen zu entwickeln - immer mit einem strukturierten Vorgehen, großer Sorgfalt und einem ausgeprägten Blick für Details.
Derzeit studiere ich User Experience Design an der <a target="_blank" href="https://thi.de" class="link link-active underline">Technischen Hochschule Ingolstadt</a>.
In meinem Studium wie auch in meinem eigenen Schaffen lege ich großen Wert auf Verlässlichkeit, Teamarbeit und einen verantwortungsvollen Umgang mit sensiblen Daten.
Neben dem Studium musiziere ich, fahre gerne Rad und game, natürlich alles auch mit Freunden.<br>
<br>
Ich freue mich, wenn du dir einen Eindruck von meiner Arbeit verschaffst. Bei Fragen oder Interesse an einer Zusammenarbeit, melde dich gerne!
                            "#))
                        }
                    }
                }

                section #Erfahrung .sect."sect-small-start" {
                    div.content{
                        h2.heading { "Meine Erfahrung" }
                        div #Werdegang {
                            div .timeline {
                                div ."timeline-item" {
                                    span ."timeline-date" {
                                        "2023 - Heute"
                                    }
                                    div ."timeline-content" {
                                        h3 { "Technische Hochschule Ingolstadt" }
                                        p { "UX Design Studium (B.Sc.)" }
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
                            div #SkillCategories {
                                @for (i,category) in skills.iter().enumerate(){
                                    div .category{
                                        h3 ."category-title"."body-strong" { (category.0) }
                                        ul."skills-list"{
                                            @for (j,svg) in category.1.iter().enumerate() {
                                                li ."skill-icon"{
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
                }

                section #Projects .sect."accent-background"{
                    div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}
                    div.content."mb-gigantic"{
                        h2.heading{
                            "Meine Top Projekte"
                        }
                    }
                    div .content #ProjectList{
                        @for project in Page::projects(){
                            @if project.favorite {
                                (project_card_html(
                                    project_card::MarkupProps {
                                        data: project,
                                        path_to_root: page.path_to_root(),
                                        is_in_grid: true,
                                        reactive_color: false,
                                    }
                                ))
                            }
                        }
                    }
                    div .content #AllProjects {
                        div.line{}
                        a draggable="false" href=(page.path_to_root()+Page::Projekte.to_href().to_str().expect("A valid path")) class="btn secondary-btn fw-medium shadow" { span{"Alle Projekte"} }
                        div.line{}
                    }
                    div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                }

                section.sect.content {
                    ""
                }
            }
            (footer())
            script{(PreEscaped(SCRIPT))}
        }
    })
}
