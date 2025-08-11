use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;
use std::{sync::OnceLock, time::Duration};

use crate::{
    components::{
        self, Component, footer::footer, head::default_head, header::header, project_card,
        project_table::Content, scrolling_img, tooltip,
    },
    include_logo, include_public, setup_language_loader,
};

use super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "home")
}

pub const MOD_PATH: &str = module_path!();

pub const STYLE: &str = include_asset!("index.css");
pub const SCRIPT: &str = include_asset!("index.js");
pub const ICH: Link = link_public!("assets/Lebenslauf/schönes bild klein bg@0,33x.jpg");

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
        style: scroll_img_style,
        ..
    } = scrolling_img::component();
    let Component {
        html: tooltip_html,
        style: tooltip_style,
        ..
    } = tooltip::component();

    let skills: [(&str, Vec<(Content, &'static str)>); 3] = [
        (
            &loader.get("Design"),
            vec![
                (
                    html! {a.link.underline{"Adobe Illustrator"}}.into(),
                    include_logo!("adobe-illustrator-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Adobe Photoshop"}}.into(),
                    include_logo!("adobe-photoshop-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Adobe Premiere Pro"}}.into(),
                    include_logo!("adobe-premiere-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Adobe XD"}}.into(),
                    include_logo!("adobe-xd-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Figma"}}.into(),
                    include_logo!("figma-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Penpot"}}.into(),
                    include_logo!("penpot-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Blender"}}.into(),
                    include_logo!("blender-svgrepo-com.svg"),
                ),
            ],
        ),
        (
            &loader.get("ICT"),
            vec![
                (
                    html! {a.link.underline{"Microsoft Office"}}.into(),
                    include_logo!("office-1-logo-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Microsoft Windwos"}}.into(),
                    include_logo!("microsoft-windows-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Apple MacOS"}}.into(),
                    include_logo!("apple-nogb-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Linux"}}.into(),
                    include_logo!("linux-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Microsoft VSCode"}}.into(),
                    include_logo!("vs-code-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"NeoVim"}}.into(),
                    include_logo!("neovim-mark@2x.svg"),
                ),
            ],
        ),
        (
            &loader.get("languages"),
            vec![
                (
                    html! {a.link.underline{"HTML"}}.into(),
                    include_logo!("html-5-no-wordmark-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"CSS"}}.into(),
                    include_logo!("CSS Logo.svg"),
                ),
                (
                    html! {a.link.underline{"JavaScript"}}.into(),
                    include_logo!("js-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Rust"}}.into(),
                    include_logo!("rust-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Java"}}.into(),
                    include_logo!("java-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"C/C++"}}.into(),
                    include_logo!("cpp-svgrepo-com.svg"),
                ),
                (
                    html! {a.link.underline{"Python"}}.into(),
                    include_logo!("python-svgrepo-com.svg"),
                ),
            ],
        ),
    ];

    components::page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head(&core_loader.get("Home"),&loader.get("description"), page.path_to_root(lang), lang))
                link rel="stylesheet" href=(page.path_to_root(lang) + *card_style );
                // script type="module" src=(page.path_to_root(lang) + *card_script ){}
                link rel="stylesheet" href=(page.path_to_root(lang) + *scroll_img_style );
                link rel="stylesheet" href=(page.path_to_root(lang) + *tooltip_style );
                style { (PreEscaped(STYLE)) }
            }

            body{
                (header(page, lang))
                main{
                    section #Hero{
                        // picture #HeroImg{img draggable="false" src=(*placeholder_img!(600,400)) alt="";}
                        div #HeroImg{(scroll_img_html(scrolling_img::MarkupProps { img: Link((page.path_to_root(lang)+*link_public!("assets/Title-img.webp")).leak()), rows: 3, columns: 3, duration: Duration::from_secs(50) }))}
                        div ."hero-content"{
                            h1."mb-large"{
                                span."fs-large"."lh-tight"{ (loader.get("welcome")) } br;
                                span.hero."lh-normal"."fw-gigantic"{ (loader.get("details")) }
                            }
                            a draggable="false" .btn."accent-btn".shadow href="#AboutMe" { span{(loader.get("discover-more"))} }
                        }
                    }


                    section #AboutMe .sect."sect-large-start"."sect-small-end" {
                        div.content {
                            h2.heading{ span."accent-text"{ (loader.get("hi")) } " " (loader.get("greeting-title")) }
                            picture{img draggable="false" src=(page.path_to_root(lang) + *ICH) alt="";}
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
                                    @for (i,category) in skills.iter().enumerate(){
                                        div .category{
                                            h3 ."category-title"."body-strong" { (category.0) }
                                            ul."skills-list"{
                                                @for (j,svg) in category.1.iter().enumerate() {
                                                    li ."skill-icon"{
                                                        (tooltip_html(tooltip::MarkupProps{
                                                            children: html!{
                                                                (PreEscaped(svg.1
                                                                // stop svg's from changing each others styles, by having duplicate ids
                                                                .replace("url(#", &format!("url(#{i}{j}"))
                                                                .replace("id=\"", &format!("id=\"{i}{j}"))))
                                                            },
                                                            content: html!((svg.0)),
                                                            popup_align:tooltip::Align::Begin,
                                                            popup_justify: tooltip::Align::Center,
                                                            popup_begin_justify:tooltip::Align::Center,
                                                            popup_begin_align:tooltip::Align::Center}))
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
                                (loader.get("top-projects"))
                            }
                        }
                        div .content #ProjectList{
                            @for project in Page::projects(){
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
