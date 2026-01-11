use Props::with_props;
use maud::{Markup, PreEscaped, Render, html};

use crate::{
    Link,
    color::CssColor,
    components::Component,
    link_public,
};

#[with_props]
pub fn markup(content: PreEscaped<String>, color: CssColor, bg_color: CssColor) -> Markup {
    html! {
        span.badge style=(format!(r#"
        --color:{};
        --bg-color:{};
        "#,color.to_string(),bg_color.to_string())){
            (content)
        }
    }
}
pub fn style() -> Link {
    link_public!("components/badge.css")
}

pub fn component() -> Component<MarkupProps, Link, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}

pub enum Badge {
    WDWU,
    ProduktDesign,
    PcGraph,
    PMMI,
    ProjektManagement,
}
impl Badge {
    pub fn into_markup_props(&self) -> MarkupProps {
        match self {
            Badge::WDWU => MarkupProps {
                content: PreEscaped("WDWU".to_string()),
                color: CssColor::Var("--white"),
                bg_color: CssColor::Calc("linear-gradient(to Bottom,var(--accent-light),var(--accent-dark))"),
            },
            Badge::ProduktDesign => MarkupProps {
                content: PreEscaped("ProduktDesign".to_string()),
                color: CssColor::Var("--white"),
                bg_color: CssColor::Var("--accent"),
            },
            Badge::PcGraph => MarkupProps {
                content: PreEscaped("PcGraph".to_string()),
                color: CssColor::Var("--white"),
                bg_color: CssColor::Var("--accent"),
            },
            Badge::PMMI => MarkupProps {
                content: PreEscaped("PMMI".to_string()),
                color: CssColor::Var("--white"),
                bg_color: CssColor::Var("--accent"),
            },
            Badge::ProjektManagement => MarkupProps {
                content: PreEscaped("ProjektManagement".to_string()),
                color: CssColor::Var("--white"),
                bg_color: CssColor::Var("--accent"),
            },
        }
    }
}

impl Render for Badge {
    fn render(&self) -> Markup {
        markup(self.into_markup_props())
    }
}
