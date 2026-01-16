use std::{fmt::Display, sync::Arc};

use Props::with_props;
use maud::{Markup, html};

use crate::{assets::stylesheet::StyleSheet, components::Component};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Percentage(u8);

impl Percentage {
    #[allow(unused)]
    /// Erstellt ein neues `Percentage`, wenn der Wert im gültigen Bereich liegt.
    /// Der Wert muss (inklusive) zwischen 0 und 100 liegen.
    pub const fn new(value: u8) -> Option<Self> {
        if value <= 100 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// # Safety
    /// Der Wert muss zwischen 0 und 100 liegen.
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        Self(value)
    }

    #[allow(unused)]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Percentage {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 100 {
            Ok(Percentage(value))
        } else {
            Err("Wert muss zwischen 0 und 100 liegen.")
        }
    }
}
impl Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

#[allow(unused)]
pub enum Align {
    Begin,
    Center,
    End,
    // Other(Percentage)
}
impl Align {
    pub const fn as_percent(&self) -> Percentage {
        match self {
            Align::Begin => unsafe { Percentage::new_unchecked(0) },
            Align::Center => unsafe { Percentage::new_unchecked(50) },
            Align::End => unsafe { Percentage::new_unchecked(100) },
            // Align::Other(percentage) => *percentage,
        }
    }
}

/// FIXME: Currently Alignment is only guaranteed to work when Centered
/// as it moves based on content- not children-size
#[with_props]
pub fn markup(
    children: Markup,
    content: Markup,
    popup_align: Align,
    popup_justify: Align,
    popup_begin_align: Align,
    popup_begin_justify: Align,
) -> Markup {
    html! {
        div."tool-tip"{
            div."tt-children"{(children)}
            div."tt-content"  style=(
                format!(
                    "--popup-align: {}; --popup-justify: {}; --popup-begin-align: {}; --popup-begin-justify: {};",
                    popup_align.as_percent(),
                    popup_justify.as_percent(),
                    popup_begin_align.as_percent(),
                    popup_begin_justify.as_percent(),
                )
            ){(content)}
        }
    }
}

pub fn style() -> Arc<StyleSheet> {
    // link_public!("components/tooltip.css")
    StyleSheet::new("public", "components/tooltip.css").unwrap()
}

pub fn component() -> Component<MarkupProps, Arc<StyleSheet>, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
