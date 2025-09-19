use std::sync::OnceLock;

use i18n_embed::fluent::FluentLanguageLoader;
use maud::PreEscaped;

use crate::{
    TabIndex,
    components::{
        Component,
        footer::footer,
        head::default_head,
        header::header,
        icon::{Icon, IconToMarkup},
        img::{self, img},
        page, phone_border,
        project_table::{self, with_sub_heading},
        tooltip,
    },
    projects::ProjectMetadata,
    setup_language_loader,
};

use super::super::*;

pub static LANGUAGE_LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

pub fn get_language_loader() -> &'static FluentLanguageLoader {
    setup_language_loader(&LANGUAGE_LOADER, "styles")
}

const DESCRIPTION: &str =
    r#"Ein Experiment, welches die Wichtigkeit eines Ansprechenden Visual Designs zeigt."#;
const CONTENT: PreEscaped<&'static str> = PreEscaped(
    r#"
Im Rahmen dieses Projekts habe ich einen bestehenden Screen einer App oder Website analysiert, 
nachgebaut und anschließend in drei unterschiedlichen UI-Stilen neugestaltet. 
Ziel war es, verschiedene Designtrends zu untersuchen und deren Wirkung auf unterschiedliche Zielgruppen zu reflektieren. 
Neben der gestalterischen Umsetzung lag der Fokus auf der stilistischen Recherche, 
einer Zielgruppenanalyse sowie der fundierten Begründung des Designprozesses.
"#,
);
pub const MOD_PATH: &str = module_path!();
pub fn meta_data(_lang: &LanguageIdentifier) -> ProjectMetadata {
    ProjectMetadata {
        page: Page::Styles,
        title_img: link_public!("assets/Screendesign/Styles/title-img.webp").into(),
        name: "Themen & Stile",
        description: DESCRIPTION,
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

    page::page(
        page.path_to_root(lang),
        html! {
            head{
                (default_head("Style",DESCRIPTION,page,lang))
                link rel="stylesheet" href=(page.path_to_root(lang) + *table_style );
                link rel="stylesheet" href=(page.path_to_root(lang) + *tooltip::style() );
                link rel="stylesheet" href=(page.path_to_root(lang) + *phone_style );
                style{(PreEscaped(include_asset!("styles.css")))}
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
                        // title: "UI-Stile im Screendesign".into(),
                        title: with_sub_heading("UI Themen & Stile","Screendesign"),
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
                            ("Hochschule", "Technische Hochschule Ingolstadt").into(),
                        ],
                        text: CONTENT.into()
                    }))
                    section.sect.content{
                        h2.heading{ "Konzentration" }
                        p { "
                            Screendesigns lassen sich häufig in verschiedene Stile, Themen und Trends unterteilen. Diese Veränderungen betreffen in erster Linie die visuelle, nicht jedoch die interaktive Komponente des Designs. Um dies zu veranschaulichen, wurde das ursprüngliche Screendesign zunächst auf seine interaktiven Elemente hin analysiert, reduziert und anschließend in verschiedene andere Themen und Stile umgewandelt.

                            Diese Version, die das Layout-Design und den tatsächlichen Content ohne das vollständige Screendesign enthält, dient als Grundversion für die nachfolgenden Screendesigns. Es werden keine grundlegenden Änderungen am Layout vorgenommen, sondern lediglich neue Screendesigns hinzugefügt.
                        " }
                        div{
                            (phone_html(phone_border::MarkupProps {
                                content: img(img::ImgProps {
                                    pre_src: page.path_to_root(lang),
                                    src: link_public!("assets/Screendesign/Styles/Tim_Ruland_Styles_Screendesign_Original-06.webp"),
                                    style:Some("background-color:var(--light);"), ..Default::default()
                                }),
                                path_to_root: page.path_to_root(lang)
                            }))
                            (phone_html(phone_border::MarkupProps {
                                content: img(img::ImgProps {
                                    pre_src: page.path_to_root(lang),
                                    src: link_public!("assets/Screendesign/Styles/Tim_Ruland_Styles_Screendesign_Original Pic.webp"),
                                    ..Default::default()
                                }),
                                path_to_root: page.path_to_root(lang)
                            }))
                        }
                    }
                }
                (footer(lang))
            }
        },
    )
}
