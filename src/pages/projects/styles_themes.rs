use std::sync::OnceLock;

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    TabIndex, assets::img::{Img, ImgProps}, components::{
        Component,
        footer::footer,
        head::default_head,
        header::header,
        icon::{Icon, IconToMarkup},
        page, phone_border,
        project_table::{self, with_sub_heading},
        tooltip,
    }, include_public, projects::ProjectMetadata, setup_language_loader
};

use super::super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "styles")
}

pub const MOD_PATH: &str = module_path!();
pub fn meta_data(lang: &LanguageIdentifier) -> ProjectMetadata {
    let loader = get_language_loader().select_languages(&[lang]);
    ProjectMetadata {
        page: Page::Styles,
        title_img: 
        // link_public!("assets/Screendesign/Styles/title-img.webp")
        Img::new("public","assets/Screendesign/Styles/title-img.webp","").unwrap()
        .into(),
        name: loader.get("name").leak(),
        description: loader.get("description").leak(),
        category: projects::Category::Screendesign,
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
        html: phone_html,
        style: phone_style,
        ..
    } = phone_border::component();

    let dark_title_img = meta_data.title_img.dark();
    let light_title_img = meta_data.title_img.light();


    let original_img = Img::new("public", "assets/Screendesign/Styles/Tim_Ruland_Styles_Screendesign_Original-06.webp", "").unwrap();
    let clear_img = Img::new("public", "assets/Screendesign/Styles/Tim_Ruland_Styles_Screendesign_Original Pic.webp", "").unwrap();
    let glass_img = Img::new("public", "assets/Screendesign/Styles/Tim_Ruland_Styles_Screendesign_Glas.webp", "").unwrap();
    let bau_img = Img::new("public", "assets/Screendesign/Styles/Tim_Ruland_Styles_Screendesign_Bauhaus.webp", "").unwrap();

    page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head("Style",loader.get("description").leak(),page,lang))
                link rel="stylesheet" href=(page.path_to_root(lang) + *table_style );
                link rel="stylesheet" href=(page.path_to_root(lang) + *tooltip::style() );
                link rel="stylesheet" href=(page.path_to_root(lang) + *phone_style );
                style{(PreEscaped(include_asset!("styles.css")))}
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
                        // title: "UI-Stile im Screendesign".into(),
                        title: with_sub_heading(loader.get("name").leak(),"Screendesign"),
                        graphic: html!{
                            picture{
                                img loading="lazy" draggable="false" id="OriginalImage"
                                    src=(page.path_to_root(lang)+*link_public!("/assets/Screendesign/Styles/Tim_Ruland_Styles_Screendesign_Original_with_new.webp"))
                                    data-source="https://medium.muz.li/weekly-design-inspiration-368-273380298382" alt="";
                            }
                        }.into(),
                        rows:&[
                            (&*core_loader.get("module").leak(), "Gestaltung").into(),
                            (&*core_loader.get("period").leak(), format!("{} 2023 - {} 2024",core_loader.get("October"),core_loader.get("February")).leak()).into(),
                            // ("Tools", "Illustrator, git, GitHub").into(),
                            (&*core_loader.get("tools").leak(), html!{ul."icon-row"{([
                                Icon::Illustrator,
                                Icon::Git,
                                Icon::GitHub,
                                ].to_markup(&page.path_to_root(lang)))}}).into(),
                            (&*core_loader.get("university").leak(), "Technische Hochschule Ingolstadt").into(),
                        ],
                        text: loader.get("content").leak().into(),
                        long_text: true
                    }))
                    section.sect.content #OldSect{
                        h2.heading{ (loader.get("concentration")) }
                        p { (loader.get("concentration-text")) }
                        div{
                            (phone_html(phone_border::MarkupProps {
                                content: original_img.render(ImgProps {
                                    path_to_root: &page.path_to_root(lang),
                                    style:Some("background-color:var(--light);"), ..Default::default()
                                }),
                                eager:false,
                                path_to_root: page.path_to_root(lang)
                            }))
                            (phone_html(phone_border::MarkupProps {
                                content: clear_img.render(ImgProps {
                                    path_to_root: &page.path_to_root(lang),
                                    style:Some("background-color:var(--light);"), ..Default::default()
                                }),
                                eager:false,
                                path_to_root: page.path_to_root(lang)
                            }))
                        }
                    }
                    section.sect.content."accent-background" #Glass  style="
                        --accent-bg-c: var(--black);
                        --bg-light: #e2d279ff;
                        --bg-normal: #F0D439;
                        --bg-dark: #F0BE3A;"{
                        div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}
                        div.desc{
                            h2.heading{ (loader.get("glass-morphism")) }
                            p.sub { (loader.get("glass-morphism-sub")) }
                            p {  (loader.get("glass-morphism-text")) }
                            ul.list {
                                li { (loader.get("glass-morphism-list-1")) }
                                li { (loader.get("glass-morphism-list-2")) }
                                li { (loader.get("glass-morphism-list-3")) }
                                li { (loader.get("glass-morphism-list-4")) }
                            }
                            div{
                                (phone_html(phone_border::MarkupProps {
                                    content: glass_img.render(ImgProps {
                                        path_to_root: &page.path_to_root(lang),
                                        style:Some("background-color:var(--light);"), ..Default::default()
                                    }),
                                    eager:false,
                                    path_to_root: page.path_to_root(lang)
                                }))
                            }
                        }
                        // div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                    }
                    section.sect.content."accent-background" #Bau style="
                        --accent-bg-c: var(--white);
                        --bg-light: #c879e2ff;
                        --bg-normal: #c239f0ff;
                        --bg-dark: #ac33caff;"{
                        // div .cut."top-cut" {(PreEscaped(include_public!("assets/noise/wave.svg")))}
                        div.desc{
                            h2.heading{ (loader.get("bau")) }
                            p.sub { (loader.get("bau-sub")) }
                            p { (loader.get("bau-text")) }
                            ul.list {
                                li { (loader.get("bau-list-1")) }
                                li { (loader.get("bau-list-2")) }
                                li { (loader.get("bau-list-3")) }
                            }
                            div{
                                (phone_html(phone_border::MarkupProps {
                                    content: bau_img.render(ImgProps {
                                        path_to_root: &page.path_to_root(lang),
                                        style:Some("background-color:var(--light);"), ..Default::default()
                                    }),
                                    eager:false,
                                    path_to_root: page.path_to_root(lang)
                                }))
                            }
                        }
                        div .cut."bot-cut" {(PreEscaped(include_public!("assets/noise/waves-opacity.svg")))}
                    }
                    section.sect{}
                }
                (footer(lang))
            }
        },
    )
}
