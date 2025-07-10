use std::{collections::BTreeMap, path::Path};

use maud::{PreEscaped, html};
use strum::IntoEnumIterator;

use crate::{capitalize, components::theme_select::theme_select, include_asset, Page, GIT_HUB_ICON};

fn group_pages() -> BTreeMap<Option<String>, Vec<Page>> {
    let mut grouped: BTreeMap<Option<String>, Vec<Page>> = BTreeMap::new();
    for page in Page::iter() {
        let parent = page.to_href().parent().and_then(|p| {
            if p == Path::new(".") {
                None
            } else {
                Some(p.display().to_string())
            }
        });
        grouped.entry(parent).or_default().push(page);
    }
    grouped
}

pub fn header(current_page: Page) -> maud::Markup {
    let underline = |page: Page| {
        if current_page.to_href() == page.to_href() {
            "underline-active"
        } else {
            ""
        }
    };

    let nav_link = |page: Page| {
        html! {
            a href=(current_page.path_to_root() + &page.to_href().display().to_string() + "#")
              class=(format!("link underline {}", underline(page))) {
                (page)
            }
        }
    };

    // Gruppiere Seiten nach Ordner (oder None für Top-Level)
    let grouped = group_pages();
    let mut groups = Vec::new();
    let group_name = |name: &str| {
        (
            format!("{}Group", capitalize(&name)),
            format!("{}Children", capitalize(&name)),
        )
    };

    html! {
        form #ShowMobileNavForm ."visually-hidden"{
            input type="checkbox" #ShowMobileNav;
        }
        label for="ShowMobileNav" #BurgerToggle .btn."secondary-btn".shadow aria-label="Menü öffnen" {
            (PreEscaped(include_asset!("icons/burger.svg")))
        }
        header #SiteHeader {
            nav {
                menu #NavLinks {
                    @for (parent, pages) in &grouped {
                        @if let Some(folder) = parent && !folder.is_empty() {
                            details.dismiss."nav-group" #(group_name(&folder).0) for=(group_name(&folder).1) role="group"{
                                summary.link.underline role="button" { (capitalize(&folder)) }
                                ({
                                    groups.push((capitalize(&folder),html!{@for page in pages {
                                        li role="menuitem" { (nav_link(*page)) }
                                    }}));
                                    ""
                                })
                                // ul role="menu" {
                                //     @for page in pages {
                                //         li role="menuitem" { (nav_link(*page)) }
                                //     }
                                // }
                            }
                        } @else {
                            @for page in pages {
                                li { (nav_link(*page)) }
                            }
                        }
                    }
                }
            }
            ul #Extern{
                li { (theme_select(current_page,&[
                        ("System", false),
                        ("Light", true),
                        ("Dark", false),
                        ("Custom", false)
                    ])) }
                li { a target="_blank" href="https://github.com/LPTimey/Tim-Portfolio" { (PreEscaped(GIT_HUB_ICON)) /*"GitHub"*/ } }
            }
            div #Groups{
                @for (name,children) in groups.iter(){
                    menu ."nav-group" #(group_name(name).1) {(children)}
                }
            }
            style{
                @for (name,_) in groups.iter(){
                    ({
                        let (group,children) = group_name(name);
                        PreEscaped(format!("#SiteHeader:has(#{group}[open]) #{children}{{opacity:1;height:fit-content;pointer-events: revert;}}"))})
                }
            }
        }
    }
}
