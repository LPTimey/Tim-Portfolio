use maud::html;

use crate::{Page, THEME_JS};

pub fn theme_select(current_page: Page, themes: &[(&str, bool)]) -> maud::Markup {
    let theme = |name: &str, default: bool| {
        html! {
            label for=(name) class="visually-hidden" { (name) }
            @if default {
                input name="theme" value=(name) id=(name) type="radio" checked;
            } @else {
                input name="theme" value=(name) id=(name) type="radio";
            }
        }
    };
    html! {
        div{
        form id="ColorPicker" class="visually-hidden" action="" {
            fieldset {
                legend class="visually-hidden" { "Pick a color scheme" }
                // (theme("System", false))
                // (theme("Light", true))
                // (theme("Dark", false))
                // (theme("Custom", false))
                @for pair in themes {
                    (theme(pair.0,pair.1))
                }
            }
        }

        // select #ThemeSelect{
        //     @for pair in themes {
        //         option value=(pair.0) selected=(pair.1) { (pair.0) }
        //     }
        // }

        details #ThemeSelect .dismiss{
            summary{
                span {
                    @for pair in themes {
                        span data-theme=(pair.0) {(pair.0)}
                    }
                }
            }
            ul {
                @for pair in themes {
                    li{label for=(pair.0) selected=(pair.1) { (pair.0) }}
                }
            }
        }

        script src=(current_page.path_to_root() + *THEME_JS) {}}
    }
}
