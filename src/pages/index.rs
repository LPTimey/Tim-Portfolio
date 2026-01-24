use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;
use std::{path::Path, sync::OnceLock};

use crate::{
    angle::Angle, assets::img::{Img, ImgProps}, components::{
        self, Component,
        footer::footer,
        head::default_head,
        header::header,
        icon::{Icon, IconToMarkup},
        project_card, scrolling_img, tooltip,
    }, include_public, setup_language_loader
};

use super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "home")
}

pub const MOD_PATH: &str = module_path!();

pub const STYLE: &str = include_asset!("index.css");
pub const SCRIPT: &str = include_asset!("index.js");

pub fn page(page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let core_loader = get_core_language_loader().select_languages(&[lang]);
    let loader = get_language_loader().select_languages(&[lang]);

    let Component {
        html: project_card_html,
        style: card_style,
        ..
    } = project_card::component();
    let Component {
        html: scroll_img_html,
        script: scroll_img_script,
        ..
    } = scrolling_img::component();
    let Component {
        html: _tooltip_html,
        style: tooltip_style,
        ..
    } = tooltip::component();

    let skills = [
        (
            loader.get("Design"),
            [
                Icon::Illustrator,
                Icon::Photoshop,
                Icon::Premiere,
                Icon::XD,
                Icon::Figma,
                Icon::Penpot,
                Icon::Blender,
            ],
        ),
        (
            loader.get("ICT"),
            [
                Icon::Office,
                Icon::Windows,
                Icon::Apple,
                Icon::Linux,
                Icon::VSCode,
                Icon::Git,
                Icon::GitHub,
            ],
        ),
        (
            loader.get("languages"),
            [
                Icon::Html,
                Icon::Css,
                Icon::JavaScript,
                Icon::Rust,
                Icon::Java,
                Icon::C,
                Icon::Python,
            ],
        ),
    ]
    .map(|(cat, icons)| (cat, icons.to_markup(&page.path_to_root(lang))));

    let title_img = Img::new(
        "public",
        Path::new("assets").join("Title-img.png"),
        "",
        false,
    )
    .unwrap();
    let profile_img = Img::new("public", "assets/Lebenslauf/schönes bild.JPG", "", false).unwrap();

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (scroll_img_script.render(&page.path_to_root(lang)));
                (default_head(&core_loader.get("Home"),&loader.get("description"), page, lang))
                (card_style.render(&page.path_to_root(lang)))
                // script type="module" src=(page.path_to_root(lang) + *card_script ){}
                (tooltip_style.render(&page.path_to_root(lang)));
                style { (PreEscaped(STYLE)) }
            }

            body{
                (header(page, lang))
                main{
                    section #Hero{
                        div #HeroImg{
                            (scroll_img_html(scrolling_img::MarkupProps {
                                img: title_img,
                                img_props: ImgProps{
                                    path_to_root:&page.path_to_root(lang),
                                    eager:true,
                                    high_prio:Some(true),
                                    sizes: Some("100vw"),
                                    ..Default::default()
                                },
                                zoom: 2.25,
                                angle: Angle::Rad(0.46),
                                speed: 0.5,
                            }))
                        }
                        div ."hero-content"{
                            h1."mb-large"{
                                span."fs-large"."fw-medium"."lh-normal"."text-shadow"{ (loader.get("welcome")) } br;
                                span.hero."text-shadow"{ (loader.get("details")) }
                            }
                            a draggable="false" .btn."accent-btn".shadow href="#AboutMe" { span{(loader.get("discover-more"))} }
                        }
                    }

                    section #AboutMe .sect."sect-large-start"."sect-small-end" {
                        div.content {
                            h2.heading{ span."accent-text"{ (loader.get("hi")) } " " (loader.get("greeting-title")) }
                            (profile_img.render(ImgProps { path_to_root:&page.path_to_root(lang), sizes:Some("40vw"),..Default::default() }))
                            p{
                                (PreEscaped(loader.get("about-me")))
                            }
                        }
                    }

                    section #Erfahrung .sect."sect-small-start" {
                        div.content{
                            h2.heading { (loader.get("Experience")) }
                            div #Werdegang {
                                div .timeline {
                                    div ."timeline-item" {
                                        span ."timeline-date" {
                                            "2025 - 2026"
                                        }
                                        div ."timeline-content" {
                                            h3 { "ADVERMA Advertising & Marketing GmbH" }
                                            p { (core_loader.get("internship")) }
                                        }
                                    }
                                    div ."timeline-item" {
                                        span ."timeline-date" {
                                            "2023 - " (loader.get("today"))
                                        }
                                        div ."timeline-content" {
                                            h3 { "Technische Hochschule Ingolstadt" }
                                            p { (loader.get("UXD")) }
                                        }
                                    }
                                    div ."timeline-item" {
                                        span ."timeline-date" {
                                            "2020 - 2022"
                                        }
                                        div ."timeline-content" {
                                            h3{ "FOS/BOS Scheyern" }
                                            p{(loader.get("FOS-tech"))}
                                        }
                                    }
                                    div ."timeline-item" {
                                        span ."timeline-date" {
                                            "2014 - 2020"
                                        }
                                        div ."timeline-content" {
                                            h3 { (loader.get("Georg-Hipp")) }
                                            p { (loader.get("math-track")) }
                                        }
                                    }
                                }
                            }
                            div #Skills{
                                div #SkillCategories {
                                    @for category in skills.iter(){
                                        div .category{
                                            h3 ."category-title"."body-strong" { (category.0) }
                                            ul."skills-list"{
                                                (category.1)
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
                                (loader.get("top-projects"))
                            }
                        }
                        div .content #ProjectList{
                            @for project in Page::projects(lang){
                                @if project.favorite {
                                    (project_card_html(
                                        project_card::MarkupProps {
                                            data: project,
                                            path_to_root: page.path_to_root(lang),
                                            is_in_grid: true,
                                            reactive_color: false,
                                            lang
                                        }
                                    ))
                                }
                            }
                        }
                        div .content #AllProjects {
                            div.line{}
                            a draggable="false" href=(page.path_to_root(lang)+Page::Projects.to_href(lang).to_str().expect("A valid path")) class="btn secondary-btn fw-medium shadow" { span{(loader.get("all-projects"))} }
                            div.line{}
                        }
                        div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                    }

                    section.sect.content {
                        ""
                    }
                }
                (footer(lang))
                script{(PreEscaped(SCRIPT))}
            }
        },
    )
}
