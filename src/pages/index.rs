use maud::PreEscaped;

use crate::{
    components::{
        self, Component, footer::footer, head::default_head, header::header, project_card,
    },
    placeholder_img,
};

use super::*;
pub const MOD_PATH: &str = module_path!();

pub const STYLE: &str = include_asset!("index.css");
pub const ICH: Link = link_public!("assets/Lebenslauf/schönes bild.JPG");

pub fn page(page: Page) -> maud::Markup {
    let Component {
        html,
        style: card_style,
        script: card_script,
    } = components::project_card::component();

    components::page::page(html! {
        head{
            (default_head("Home","TODO: Add description", page.path_to_root()))
            style { (PreEscaped(STYLE)) }
            link rel="stylesheet" href=(page.path_to_root() + *card_style );
            source type="module" src=(page.path_to_root() + *card_script ){}
        }

        body{
            (header(page))
            main{
                section id="Hero"{
                    picture id="HeroImg"{img src=(*placeholder_img!(600,400)) alt="";}
                    div ."hero-content"{
                        h1."fmb-large"{
                            span."fs-large"."lh-tight"{ "Willkommen, hier" } br;
                            span.hero."lh-normal"{ "wo die Details scheinen" }
                        }
                        a class="btn accent-btn" href="#AboutMe" { "Entdecke mehr" }
                    }
                }

                section id="AboutMe" .content.sect."sect-large-start" {
                    h2.heading{ span."accent-text"{ "Hi! " } "Ich bin Tim." }
                    picture{img src=(page.path_to_root() + *ICH) alt="";}
                    p{
                        (PreEscaped(r#"
Mich faszinieren sowohl IT & Programmierung als auch Gestaltung & Design.
Aus diesem Zusammenspiel zwischen technischer Präzision und gestalterischem Denken ziehe ich die Motivation für meine Projekte.
Es ermöglicht mir, ansprechende und zugleich effiziente Lösungen zu entwickeln - immer mit einem strukturierten Vorgehen, großer Sorgfalt und einem ausgeprägten Blick für Details.
Derzeit studiere ich User Experience Design an der Technischen Hochschule Ingolstadt.
In meinem Studium wie auch in meinem eigenen Schaffen lege ich großen Wert auf Verlässlichkeit, Teamarbeit und einen verantwortungsvollen Umgang mit sensiblen Daten.
Neben dem Studium musiziere ich, fahre gerne Rad und game, natürlich alles auch mit Freunden.

Ich freue mich, wenn du dir einen Eindruck von meiner Arbeit verschaffst. Bei Fragen oder Interesse an einer Zusammenarbeit, melde dich gerne!
                        "#))
                    }
                }

                section id="Erfahrung" .content.sect {
                    div{
                        "Laufbahn"
                    }
                    div{
                        "Skills"
                    }
                }

                section id="Projects" .sect{
                    div .content{
                        (html(project_card::MarkupProps{title:"WatchOut",description:"Eine Uhr und eine App um Menschen mit Demenz und deren Familie zu helfen ihr Leben sorgloser zu leben.",img:"",theme:"DMMS"}))
                        (html(project_card::MarkupProps{title:"Drucker Touchscreen",description:"",img:"",theme:"Screendesign"}))
                        (html(project_card::MarkupProps{title:"Themen & Stile",description:"",img:"",theme:"Screendesign"}))
                        (html(project_card::MarkupProps{title:"Tetris in Arduino & C",description:"",img:"",theme:"Programmieren"}))
                    }
                }
            }
            (footer())
        }
    })
}
