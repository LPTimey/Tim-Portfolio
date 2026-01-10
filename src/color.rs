use maud::PreEscaped;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssColor {
    Color(Color),
    Var(&'static str),
    Calc(&'static str),
}
impl ToString for CssColor {
    fn to_string(&self) -> String {
        match *self {
            CssColor::Color(color) => color.to_css().into(),
            CssColor::Var(str) => format!("var({})", str),
            CssColor::Calc(str) => str.into(),
        }
    }
}
impl<T: Into<Color>> From<T> for CssColor {
    fn from(value: T) -> Self {
        CssColor::Color(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBA(u8, u8, u8, f64);
impl From<HSLA> for RGBA {
    fn from(value: HSLA) -> Self {
        // HSL to RGB
        let h = value.0;
        let s = value.1;
        let l = value.2;
        let a = value.3;

        // Helper: hue -> rgb component
        fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
            let mut t = t;
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 1.0 / 2.0 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        }

        let h_norm = (h / 360.0) % 1.0;
        let (r_f, g_f, b_f) = if s == 0.0 {
            (l, l, l)
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            (
                hue_to_rgb(p, q, h_norm + 1.0 / 3.0),
                hue_to_rgb(p, q, h_norm),
                hue_to_rgb(p, q, h_norm - 1.0 / 3.0),
            )
        };

        // Convert to 0..255 and round
        let r = (r_f.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (g_f.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (b_f.clamp(0.0, 1.0) * 255.0).round() as u8;

        // alpha should already be valid per HSLA::new; clamp defensively
        let a = a.clamp(0.0, 1.0);

        RGBA(r, g, b, a)
    }
}
impl From<OKLABA> for RGBA {
    fn from(value: OKLABA) -> Self {
        // OKLab -> linear sRGB -> sRGB
        // From OKLab paper:
        // l_ = (L + 0.3963377774 * a + 0.2158037573 * b)
        // m_ = (L - 0.1055613458 * a - 0.0638541728 * b)
        // s_ = (L - 0.0894841775 * a - 1.2914855480 * b)
        // then cube these -> l, m, s
        // linear RGB:
        // r = +4.0767416621 * l -3.3077115913 * m +0.2309699292 * s
        // g = -1.2684380046 * l +2.6097574011 * m -0.3413193965 * s
        // b = -0.0041960863 * l -0.7034186147 * m +1.7076147010 * s
        let L = value.0;
        let A = value.1;
        let B = value.2;
        let alpha = value.3.clamp(0.0, 1.0);

        let l_ = L + 0.3963377774 * A + 0.2158037573 * B;
        let m_ = L - 0.1055613458 * A - 0.0638541728 * B;
        let s_ = L - 0.0894841775 * A - 1.2914855480 * B;

        let l = l_.powi(3);
        let m = m_.powi(3);
        let s = s_.powi(3);

        let r_lin = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
        let g_lin = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        let b_lin = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

        // linear to sRGB (compand)
        fn lin_to_srgb(c: f64) -> f64 {
            if c <= 0.0 {
                0.0
            } else if c >= 1.0 {
                1.0
            } else if c <= 0.0031308 {
                12.92 * c
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            }
        }

        let r = (lin_to_srgb(r_lin).clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (lin_to_srgb(g_lin).clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (lin_to_srgb(b_lin).clamp(0.0, 1.0) * 255.0).round() as u8;

        RGBA(r, g, b, alpha)
    }
}

impl RGBA {
    pub const fn new(r: u8, g: u8, b: u8, a: f64) -> Result<Self, RGBAError> {
        if a < 0.0 {
            return Err(RGBAError::AlphaNegative);
        }
        if a > 1.0 {
            return Err(RGBAError::AlphaTooBig);
        }
        Ok(Self(r, g, b, a))
    }

    pub const unsafe fn new_unchecked(r: u8, g: u8, b: u8, a: f64) -> Self {
        assert!(a >= 0.0 && a <= 1.0);
        Self(r, g, b, a)
    }

    // Convert to CSS RGBA format
    pub fn to_css(&self) -> String {
        format!("rgba({}, {}, {}, {:.5})", self.0, self.1, self.2, self.3)
    }
    pub const fn new_static<const R: u8, const G: u8, const B: u8, const A: u8>() -> Self {
        struct Assert<const N: u8>;
        impl<const N: u8> Assert<N> {
            const ASSERT: () = assert!(N <= 100, "invalid value");
        }
        _ = Assert::<A>::ASSERT;
        if A > 100 {
            panic!("Alpha out of range (must be 0..=100 representing 0.00..1.00)");
        }
        Self(R, G, B, A as f64 / 100.)
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSLA(f64, f64, f64, f64);
impl From<RGBA> for HSLA {
    fn from(value: RGBA) -> Self {
        let r = value.0 as f64 / 255.0;
        let g = value.1 as f64 / 255.0;
        let b = value.2 as f64 / 255.0;
        let a = value.3;

        // sRGB companding inverse (to linear) is not needed for HSL conversion;
        // HSL uses gamma-encoded sRGB directly.
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Lightness
        let l = (max + min) / 2.0;

        // Hue and saturation
        let (h, s) = if delta == 0.0 {
            (0.0, 0.0)
        } else {
            let s = if l < 0.5 {
                delta / (max + min)
            } else {
                delta / (2.0 - max - min)
            };

            let mut h = if max == r {
                (g - b) / delta + if g < b { 6.0 } else { 0.0 }
            } else if max == g {
                (b - r) / delta + 2.0
            } else {
                (r - g) / delta + 4.0
            };
            h *= 60.0;
            if h < 0.0 {
                h += 360.0;
            }
            (h % 360.0, s)
        };

        HSLA(h, s, l, a)
    }
}
impl From<OKLABA> for HSLA {
    fn from(value: OKLABA) -> Self {
        // Convert OKLab -> RGBA -> HSLA
        let rgba: RGBA = value.into();
        rgba.into()
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OKLABA(f64, f64, f64, f64);
impl From<HSLA> for OKLABA {
    fn from(value: HSLA) -> Self {
        // HSLA -> RGBA -> OKLABA
        let rgba: RGBA = value.into();
        rgba.into()
    }
}
impl From<RGBA> for OKLABA {
    fn from(value: RGBA) -> Self {
        // sRGB (0..255) -> linear RGB -> OKLab
        let r = value.0 as f64 / 255.0;
        let g = value.1 as f64 / 255.0;
        let b = value.2 as f64 / 255.0;
        let a = value.3.clamp(0.0, 1.0);

        // sRGB to linear
        fn srgb_to_lin(c: f64) -> f64 {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        let r_lin = srgb_to_lin(r);
        let g_lin = srgb_to_lin(g);
        let b_lin = srgb_to_lin(b);

        // linear RGB -> LMS
        // matrix from sRGB linear to LMS (via linear RGB -> XYZ -> LMS). Use
        // combined matrix as commonly used for OKLab conversion:
        let l = 0.4122214708 * r_lin + 0.5363325363 * g_lin + 0.0514459929 * b_lin;
        let m = 0.2119034982 * r_lin + 0.6806995451 * g_lin + 0.1073969566 * b_lin;
        let s = 0.0883024619 * r_lin + 0.2817188376 * g_lin + 0.6299787005 * b_lin;

        let l_ = l.powf(1.0 / 3.0);
        let m_ = m.powf(1.0 / 3.0);
        let s_ = s.powf(1.0 / 3.0);

        let L = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
        let A = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
        let B = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

        OKLABA(L, A, B, a)
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
