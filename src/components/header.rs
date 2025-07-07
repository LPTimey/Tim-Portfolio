use std::{collections::BTreeMap, path::Path};

use maud::{PreEscaped, html};
use strum::IntoEnumIterator;

use crate::{GIT_HUB_ICON, Page, capitalize, components::theme_select::theme_select};

pub fn header(current_page: Page) -> maud::Markup {
    let underline = |page: Page| {
        if current_page.to_href() == page.to_href() {
            "hover"
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
    let mut grouped: BTreeMap<Option<String>, Vec<Page>> = BTreeMap::new();
    for page in Page::iter() {
        let href = page.to_href();
        let parent = match href.parent() {
            Some(p) if p == Path::new(".") => None,
            Some(p) => Some(p.display().to_string()),
            None => None,
        };
        grouped.entry(parent).or_default().push(page);
    }

    html! {
        header #SiteHeader {
            nav {
                ul {
                    @for (parent, pages) in &grouped {
                        @if let Some(folder) = parent && !folder.is_empty() {
                            details.dismiss {
                                summary { (capitalize(&folder)) }
                                ul {
                                    @for page in pages {
                                        li { (nav_link(*page)) }
                                    }
                                }
                            }
                        } @else {
                            @for page in pages {
                                li { (nav_link(*page)) }
                            }
                        }
                    }
                }
            }
            ul {
                li { (theme_select(current_page,&[("System", false),
                ("Light", true),
                ("Dark", false),
                ("Custom", false)])) }
                li { a target="_blank" href="https://github.com/#TODO:AddLink" { (PreEscaped(GIT_HUB_ICON)) /*"GitHub"*/ } }
            }
        }
    }
}
