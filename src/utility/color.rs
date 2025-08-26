use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub enum Color {
    RRGGBB { rr: u8, gg: u8, bb: u8 },
    AARRGGBB { aa: u8, rr: u8, gg: u8, bb: u8 },
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{}",
            match self {
                Color::RRGGBB { rr, gg, bb } => format!("{:02X}{:02X}{:02X}", rr, gg, bb),
                Color::AARRGGBB { aa, rr, gg, bb } =>
                    format!("{:02X}{:02X}{:2X}{:02X}", aa, rr, gg, bb),
            }
        )
    }
}

impl Color {
    pub fn new_rrggbb_str(rrggbb: &str) -> Color {
        if rrggbb.len() != 6 {
            panic!("Invalid color length for rrggbb");
        }
        Color::RRGGBB {
            rr: u8::from_str_radix(&rrggbb[0..=1], 16)
                .expect(&format!("Invalid hex for RR of {}", rrggbb)),
            gg: u8::from_str_radix(&rrggbb[2..=3], 16)
                .expect(&format!("Invalid hex for GG of {}", rrggbb)),
            bb: u8::from_str_radix(&rrggbb[4..=5], 16)
                .expect(&format!("Invalid hex for BB of {}", rrggbb)),
        }
    }

    pub fn new_aarrggbb_str(aarrggbb: &str) -> Color {
        if aarrggbb.len() != 8 {
            panic!("Invalid color length for rrggbb");
        }
        Color::AARRGGBB {
            aa: u8::from_str_radix(&aarrggbb[0..=1], 16)
                .expect(&format!("Invalid hex for AA of {}", aarrggbb)),
            rr: u8::from_str_radix(&aarrggbb[2..=3], 16)
                .expect(&format!("Invalid hex for RR of {}", aarrggbb)),
            gg: u8::from_str_radix(&aarrggbb[4..=5], 16)
                .expect(&format!("Invalid hex for GG of {}", aarrggbb)),
            bb: u8::from_str_radix(&aarrggbb[6..=7], 16)
                .expect(&format!("Invalid hex for BB of {}", aarrggbb)),
        }
    }

    pub fn new_rrggb_u8(rr: u8, bb: u8, gg: u8) -> Color {
        Color::RRGGBB { rr, gg, bb }
    }

    pub fn new_aarrggb_u8(aa: u8, rr: u8, bb: u8, gg: u8) -> Color {
        Color::AARRGGBB { aa, rr, gg, bb }
    }

    pub fn rr(&self) -> u8 {
        match self {
            Color::RRGGBB { rr, .. } => *rr,
            Color::AARRGGBB { rr, .. } => *rr,
        }
    }

    pub fn gg(&self) -> u8 {
        match self {
            Color::RRGGBB { gg, .. } => *gg,
            Color::AARRGGBB { gg, .. } => *gg,
        }
    }

    pub fn bb(&self) -> u8 {
        match self {
            Color::RRGGBB { bb, .. } => *bb,
            Color::AARRGGBB { bb, .. } => *bb,
        }
    }

    pub fn aa(&self) -> Option<u8> {
        match self {
            Color::RRGGBB { .. } => None,
            Color::AARRGGBB { aa, .. } => Some(*aa),
        }
    }
}
