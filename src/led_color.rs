use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr888, Gray2, Gray4, Gray8, GrayColor, Rgb555, Rgb565, Rgb888, RgbColor,
};

#[derive(Clone, Copy)]
pub struct LedColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<Bgr555> for LedColor {
    fn from(p: Bgr555) -> Self {
        LedColor {
            red: p.r() << 3,
            green: p.g() << 3,
            blue: p.b() << 3,
        }
    }
}

impl From<Bgr565> for LedColor {
    fn from(p: Bgr565) -> Self {
        LedColor {
            red: p.r() << 3,
            green: p.g() << 2,
            blue: p.b() << 3,
        }
    }
}

impl From<Bgr888> for LedColor {
    fn from(p: Bgr888) -> Self {
        LedColor {
            red: p.r(),
            green: p.g(),
            blue: p.b(),
        }
    }
}

impl From<Gray2> for LedColor {
    fn from(p: Gray2) -> Self {
        LedColor {
            red: p.luma() << 6,
            green: p.luma() << 6,
            blue: p.luma() << 6,
        }
    }
}

impl From<Gray4> for LedColor {
    fn from(p: Gray4) -> Self {
        LedColor {
            red: p.luma() << 4,
            green: p.luma() << 4,
            blue: p.luma() << 4,
        }
    }
}

impl From<Gray8> for LedColor {
    fn from(p: Gray8) -> Self {
        LedColor {
            red: p.luma(),
            green: p.luma(),
            blue: p.luma(),
        }
    }
}

impl From<Rgb555> for LedColor {
    fn from(p: Rgb555) -> Self {
        LedColor {
            red: p.r() << 3,
            green: p.g() << 3,
            blue: p.b() << 3,
        }
    }
}

impl From<Rgb565> for LedColor {
    fn from(p: Rgb565) -> Self {
        LedColor {
            red: p.r() << 3,
            green: p.g() << 2,
            blue: p.b() << 3,
        }
    }
}

impl From<Rgb888> for LedColor {
    fn from(p: Rgb888) -> Self {
        LedColor {
            red: p.r(),
            green: p.g(),
            blue: p.b(),
        }
    }
}
