use std::{collections::BTreeMap, path::Path};

use i18n_embed::fluent::FluentLanguageLoader;
use maud::{PreEscaped, html};
use strum::IntoEnumIterator;
use unic_langid::LanguageIdentifier;

use crate::{
    capitalize, components::theme_select::theme_select, get_core_language_loader, include_asset, Page, GIT_HUB_ICON
};

fn group_pages(lang: &LanguageIdentifier, loader: &FluentLanguageLoader) -> BTreeMap<Option<String>, Vec<Page>> {
    let mut grouped: BTreeMap<Option<String>, Vec<Page>> = BTreeMap::new();
    for page in Page::iter() {
        let mut path = page.to_href(lang);
        let lang_prefix = lang.to_string();

        if path.starts_with(&lang_prefix) {
            path = path.strip_prefix(&lang_prefix).unwrap_or(&path).to_path_buf();
        }

        let parent = path.parent().and_then(|p| {
            if p == Path::new(".") {
                None
            } else {
                let string = p.display().to_string();
                if loader.has(&string){
                    Some(loader.get(&string))
                }else{
                    Some(string)
                }
            }
        });
        grouped.entry(parent).or_default().push(page);
    }
    grouped
}

pub fn header(current_page: Page, lang: &LanguageIdentifier) -> maud::Markup {
    let loader = get_core_language_loader().select_languages(&[lang]);
    let underline = |page: Page| {
        if current_page.to_href(lang) == page.to_href(lang) {
            "underline-active"
        } else {
            ""
        }
    };

    let nav_link = |page: Page| {
        html! {
            a draggable="false" href=(current_page.path_to_root(lang) + &page.to_href(lang).display().to_string() + "#")
              class=(format!("link underline {}", underline(page))) {
                (page.to_localized_string(lang))
            }
        }
    };

    // Gruppiere Seiten nach Ordner (oder None für Top-Level)
    let grouped = group_pages(lang,&loader);
    let mut groups = Vec::new();
    let group_name = |name: &str| {
        (
            format!("{}Group", capitalize(name)),
            format!("{}Children", capitalize(name)),
        )
    };

    html! {
        form #ShowMobileNavForm ."visually-hidden"{
            label for="ShowMobileNav" {"Toggle Nav"}
            input type="checkbox" #ShowMobileNav;
        }
        label for="ShowMobileNav" #BurgerToggle .btn."secondary-btn".shadow aria-label="Menü öffnen" {
            (PreEscaped(include_asset!("icons/burger.svg")))
        }
        header #SiteHeader {
            nav {
                menu #NavLinks{
                    @for (parent, pages) in &grouped {
                        @if let Some(folder) = parent && !folder.is_empty() {
                            details.dismiss."nav-group" #(group_name(folder).0) for=(group_name(folder).1){
                                summary.link.underline { (capitalize(&folder)) }
                                ({
                                    groups.push((capitalize(folder),html!{@for page in pages {
                                        li { (nav_link(*page)) }
                                    }}));
                                    ""
                                })
                                // ul {
                                //     @for page in pages {
                                //         li { (nav_link(*page)) }
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
                    ], lang)) }
                li { a draggable="false" target="_blank" href="https://github.com/LPTimey/Tim-Portfolio" aria-label="Mein Github"{ (PreEscaped(GIT_HUB_ICON)) /*"GitHub"*/ } }
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
