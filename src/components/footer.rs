use maud::html;

pub fn footer() -> maud::Markup {
    html! {
        div.grow{}
        footer #SiteFooter{}
    }
}
