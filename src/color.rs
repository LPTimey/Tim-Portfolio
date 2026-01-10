use maud::PreEscaped;
use thiserror::Error;

pub enum Color {
    RGBA(RGBA),
    HSLA(HSLA),
    OKLABA(OKLABA),
}
impl Color {
    pub fn to_css(&self) -> PreEscaped<String> {
        match self {
            Color::RGBA(rgba) => PreEscaped(rgba.to_css()),
            Color::HSLA(hsla) => PreEscaped(hsla.to_css()),
            Color::OKLABA(oklaba) => PreEscaped(oklaba.to_css()),
        }
    }
    pub fn to_rgba(self) -> Self {
        match self {
            Color::RGBA(_) => self,
            Color::HSLA(hsla) => Self::RGBA(hsla.into()),
            Color::OKLABA(oklaba) => Self::RGBA(oklaba.into()),
        }
    }
    pub fn to_hsla(self) -> Self {
        match self {
            Color::RGBA(rgba) => Self::HSLA(rgba.into()),
            Color::HSLA(_) => self,
            Color::OKLABA(oklaba) => Self::HSLA(oklaba.into()),
        }
    }
    pub fn to_oklaba(self) -> Self {
        match self {
            Color::RGBA(rgba) => Self::OKLABA(rgba.into()),
            Color::HSLA(hsla) => Self::OKLABA(hsla.into()),
            Color::OKLABA(_) => self,
        }
    }
}
impl From<RGBA> for Color {
    fn from(value: RGBA) -> Self {
        Self::RGBA(value)
    }
}
impl From<HSLA> for Color {
    fn from(value: HSLA) -> Self {
        Self::HSLA(value)
    }
}
impl From<OKLABA> for Color {
    fn from(value: OKLABA) -> Self {
        Self::OKLABA(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum RGBAError {
    #[error("Alpha value cannot be negative")]
    AlphaNegative,
    #[error("Alpha value is too big (must be ≤ 1)")]
    AlphaTooBig,
}

pub struct RGBA(u8, u8, u8, f64);
impl From<HSLA> for RGBA {
    fn from(value: HSLA) -> Self {
        todo!()
    }
}
impl From<OKLABA> for RGBA {
    fn from(value: OKLABA) -> Self {
        todo!()
    }
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: f64) -> Result<Self, RGBAError> {
        if a < 0.0 {
            return Err(RGBAError::AlphaNegative);
        }
        if a > 1.0 {
            return Err(RGBAError::AlphaTooBig);
        }
        Ok(Self(r, g, b, a))
    }

    pub unsafe fn new_unchecked(r: u8, g: u8, b: u8, a: f64) -> Self {
        assert!(a >= 0.0 && a <= 1.0);
        Self(r, g, b, a)
    }

    // Convert to CSS RGBA format
    pub fn to_css(&self) -> String {
        format!("rgba({}, {}, {}, {:.5})", self.0, self.1, self.2, self.3)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum HSLAError {
    #[error("Hue value is too big (must be < 360)")]
    HueTooBig,
    #[error("Saturation value is too big (must be ≤ 1)")]
    SatTooBig,
    #[error("Lightness value is too big (must be ≤ 1)")]
    LightTooBig,
    #[error("Alpha value is too big (must be ≤ 1)")]
    AlphaTooBig,
    #[error("Hue value cannot be negative")]
    HueNegative,
    #[error("Saturation value cannot be negative")]
    SatNegative,
    #[error("Lightness value cannot be negative")]
    LightNegative,
    #[error("Alpha value cannot be negative")]
    AlphaNegative,
}

pub struct HSLA(f64, f64, f64, f64);
impl From<RGBA> for HSLA {
    fn from(value: RGBA) -> Self {
        todo!()
    }
}
impl From<OKLABA> for HSLA {
    fn from(value: OKLABA) -> Self {
        todo!()
    }
}

impl HSLA {
    pub fn new(h: f64, s: f64, l: f64, a: f64) -> Result<Self, HSLAError> {
        if h < 0.0 {
            return Err(HSLAError::HueNegative);
        }
        if h >= 360.0 {
            return Err(HSLAError::HueTooBig);
        }
        if s < 0.0 {
            return Err(HSLAError::SatNegative);
        }
        if s > 1.0 {
            return Err(HSLAError::SatTooBig);
        }
        if l < 0.0 {
            return Err(HSLAError::LightNegative);
        }
        if l > 1.0 {
            return Err(HSLAError::LightTooBig);
        }
        if a < 0.0 {
            return Err(HSLAError::AlphaNegative);
        }
        if a > 1.0 {
            return Err(HSLAError::AlphaTooBig);
        }
        Ok(Self(h, s, l, a))
    }

    pub unsafe fn new_unchecked(h: f64, s: f64, l: f64, a: f64) -> Self {
        assert!(h >= 0.0 && h < 360.0);
        assert!(s >= 0.0 && s <= 1.0);
        assert!(l >= 0.0 && l <= 1.0);
        assert!(a >= 0.0 && a <= 1.0);
        Self(h, s, l, a)
    }

    // Convert to CSS HSLA format
    pub fn to_css(&self) -> String {
        format!(
            "hsl({:.2} {:.2}% {:.2}% / {:.5})",
            self.0,
            self.1 * 100.0,
            self.2 * 100.0,
            self.3
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum OKLABAError {
    #[error("Alpha value cannot be negative")]
    AlphaNegative,
    #[error("Alpha value is too big (must be ≤ 1)")]
    AlphaTooBig,
    #[error("Lightness value is too low (must be ≥ 0)")]
    LTooLow,
    #[error("Lightness value is too high (must be ≤ 1)")]
    LTooHigh,
    #[error("A value is too low (must be ≥ -1)")]
    ATooLow,
    #[error("A value is too high (must be ≤ 1)")]
    ATooHigh,
    #[error("B value is too low (must be ≥ -1)")]
    BTooLow,
    #[error("B value is too high (must be ≤ 1)")]
    BTooHigh,
}

pub struct OKLABA(f64, f64, f64, f64);
impl From<HSLA> for OKLABA {
    fn from(value: HSLA) -> Self {
        todo!()
    }
}
impl From<RGBA> for OKLABA {
    fn from(value: RGBA) -> Self {
        todo!()
    }
}

impl OKLABA {
    pub fn new(l: f64, a: f64, b: f64, alpha: f64) -> Result<Self, OKLABAError> {
        if alpha < 0.0 {
            return Err(OKLABAError::AlphaNegative);
        }
        if alpha > 1.0 {
            return Err(OKLABAError::AlphaTooBig);
        }
        // Define valid ranges for l, a, and b
        if l < 0.0 {
            return Err(OKLABAError::LTooLow);
        }
        if l > 1.0 {
            return Err(OKLABAError::LTooHigh);
        }
        if a < -1.0 {
            return Err(OKLABAError::ATooLow);
        }
        if a > 1.0 {
            return Err(OKLABAError::ATooHigh);
        }
        if b < -1.0 {
            return Err(OKLABAError::BTooLow);
        }
        if b > 1.0 {
            return Err(OKLABAError::BTooHigh);
        }
        Ok(Self(l, a, b, alpha))
    }

    pub unsafe fn new_unchecked(l: f64, a: f64, b: f64, alpha: f64) -> Self {
        assert!(alpha >= 0.0 && alpha <= 1.0);
        assert!(l >= 0.0 && l <= 1.0);
        assert!(a >= -1.0 && a <= 1.0);
        assert!(b >= -1.0 && b <= 1.0);
        Self(l, a, b, alpha)
    }

    // Convert to CSS OKLABA format
    pub fn to_css(&self) -> String {
        format!("oklab({} {} {} / {:.5})", self.0, self.1, self.2, self.3)
    }
}
