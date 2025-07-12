use maud::PreEscaped;

use crate::{
    components::{
        Component,
        footer::footer,
        head::default_head,
        header::header,
        project_table::{self},
    },
    projekte::ProjectMetadata,
};

use super::super::*;

const DESCRIPTION: &str = r#"Eine Uhr und eine App um Menschen mit Demenz und deren Familie zu helfen ihr Leben sorgloser zu leben."#;
const CONTENT: PreEscaped<&'static str> = PreEscaped(r#"
<p> Demenz hat tiefgreifende Auswirkungen auf das Leben der Betroffenen und ihrer
    Familienangehörigen.
    Watch out unterstützt die Angehörigen dabei, einen klaren Überblick über die Situation zu
    behalten und schnell eingreifen zu können.
</p>
<p>
    Watch Out besteht aus zwei Teilen: Uhr & Begleitapp.
</p>
<p>
    Da Demenz oft die vertrauten Gewohnheiten und Erinnerungen der Betroffenen am längsten
    bewahrt, wurde die Uhr im klassischen, analogen Design gestaltet. Sie sendet GPS-Daten,
    verfügt
    über eine aktive Fallerkennung und eine
    Notruffunktion mit den 2 Knöpfen. Zusätzlich behält sie ihre Funktion als gewöhnliche
    Analoguhr
    mit Krone bei.
</p>
<p>
    Die App ist simpel und minimal für leichte und schnelle Nutzung.
    Sie bietet einen Überblick, Benachrichtigen & Notruffunktionen.
</p>
"#);
pub const MOD_PATH: &str = module_path!();
pub fn meta_data() -> ProjectMetadata {
    ProjectMetadata {
        title_img: Box::new(link_public!(
                (path_to_root(mod_path_to_href(MOD_PATH).expect("A valid path").as_path()) + 
                "assets/Design der Mensch Maschine Schnittstelle/WatchOut/title-img-flipp-bg.webp")
                .leak()
            )),
        name: "Watch Out",
        description: DESCRIPTION,
        category: projekte::Category::DMMS,
        favorite: true,
        path: mod_path_to_href(MOD_PATH).expect("A valid path"),
    }
}

pub fn page(page: Page) -> maud::Markup {
    let Component {
        html: table_html,
        style: table_style,
        ..
    } = project_table::component();

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
        ("Tools", "DMMS").into(),
        ("Hochschule", "Technische Hochschule Ingolstadt").into(),
    ];

    let video_href = page.path_to_root()
        + *link_public!("assets/Design der Mensch Maschine Schnittstelle/WatchOut/Video720_1.mp4");

    html! {
        head{
            (default_head("Watchout","TODO: Add description",page.path_to_root()))
            link rel="stylesheet" href=(page.path_to_root()+*table_style);
        }

        body{
            (header(page))
            main{
                section #Hero{
                    picture #HeroImg{img draggable="false" src=(page.path_to_root()+*meta_data().title_img.light()) alt="";}
                    div ."hero-content"{
                        h1."mb-large"{
                            span."fs-large"."lh-tight"{ "Willkommen, hier" } br;
                            span.hero."lh-normal"."fw-gigantic"{ "wo die Details scheinen" }
                        }
                        a draggable="false" .btn."accent-btn".shadow href="#AboutMe" { "Entdecke mehr" }
                    }
                }
                (table_html(project_table::MarkupProps {
                    title: "Watchout: Motivation & Generelles".into(),
                    graphic: html!{
                        video controls{
                            source src=(video_href) type="video/mp4";
                            a href=(video_href) type="video/mp4"{}
                        }
                    }.into(),
                    rows,
                    text: CONTENT.into()
                }))
            }
            (footer())
        }
    }
}
