use Props::with_props;
use maud::{Markup, html};

use crate::{components::Component, link_public, Link};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Percentage(u8);

impl Percentage {
    /// Erstellt ein neues `Percentage`, wenn der Wert im gültigen Bereich liegt.
    pub fn new(value: u8) -> Option<Self> {
        if value <= 100 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn get(self) -> u8 {
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

pub enum Align {
    Begin,
    Center,
    End,
}

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
            div."tt-content"{(content)}
        }
    }
}

pub fn style() -> Link {
    link_public!("components/tooltip.css")
}

pub fn component() -> Component<MarkupProps, ()> {
    Component {
        html: markup,
        style: style(),
        script: (),
    }
}
