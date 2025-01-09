use crossterm;

#[repr(C)]
pub enum crossterm_color_type {
    BLACK_COLOR,
    WHITE_COLOR,

    RED_COLOR,
    GREEN_COLOR,
    BLUE_COLOR,
    YELLOW_COLOR,
    MAGENTA_COLOR,
    CYAN_COLOR,
    GREY_COLOR,

    DARK_RED_COLOR,
    DARK_GREEN_COLOR,
    DARK_BLUE_COLOR,
    DARK_YELLOW_COLOR,
    DARK_MAGENTA_COLOR,
    DARK_CYAN_COLOR,
    DARK_GREY_COLOR,

    ANSI_COLOR,
    RGB_COLOR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crossterm_ansi_color {
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crossterm_rgb_color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
pub union crossterm_color_value {
    pub ansi: crossterm_ansi_color,
    pub rgb: crossterm_rgb_color,
}

#[repr(C)]
pub struct crossterm_color {
    pub t: crossterm_color_type,
    pub v: crossterm_color_value,
}

pub fn from_ffi_color_to_rust_color(color: &crossterm_color) -> crossterm::style::Color {
    use crossterm::style::Color;
    use crossterm_color_type::*;

    unsafe {
        match color.t {
            BLACK_COLOR => Color::Black,
            WHITE_COLOR => Color::White,
            RED_COLOR => Color::Red,
            GREEN_COLOR => Color::Green,
            BLUE_COLOR => Color::Blue,
            YELLOW_COLOR => Color::Yellow,
            MAGENTA_COLOR => Color::Magenta,
            CYAN_COLOR => Color::Cyan,
            GREY_COLOR => Color::Grey,
            DARK_RED_COLOR => Color::DarkRed,
            DARK_GREEN_COLOR => Color::DarkGreen,
            DARK_BLUE_COLOR => Color::DarkBlue,
            DARK_YELLOW_COLOR => Color::DarkYellow,
            DARK_MAGENTA_COLOR => Color::DarkMagenta,
            DARK_CYAN_COLOR => Color::DarkCyan,
            DARK_GREY_COLOR => Color::DarkGrey,
            ANSI_COLOR => Color::AnsiValue(color.v.ansi.value),
            RGB_COLOR => Color::Rgb {
                r: color.v.rgb.r,
                g: color.v.rgb.g,
                b: color.v.rgb.b,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::crossterm_ansi_color;
    use super::crossterm_color;
    use super::crossterm_color_type::*;
    use super::crossterm_color_value;
    use super::crossterm_rgb_color;
    use super::from_ffi_color_to_rust_color;

    use crossterm::style::Color;

    #[test]
    fn black_color_cast() {
        let ffi_color = crossterm_color {
            t: BLACK_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::Black);
    }

    #[test]
    fn white_color_cast() {
        let ffi_color = crossterm_color {
            t: WHITE_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::White);
    }

    #[test]
    fn red_color_cast() {
        let ffi_color = crossterm_color {
            t: RED_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::Red);
    }

    #[test]
    fn green_color_cast() {
        let ffi_color = crossterm_color {
            t: GREEN_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::Green);
    }

    #[test]
    fn blue_color_cast() {
        let ffi_color = crossterm_color {
            t: BLUE_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::Blue);
    }

    #[test]
    fn yellow_color_cast() {
        let ffi_color = crossterm_color {
            t: YELLOW_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::Yellow);
    }

    #[test]
    fn magenta_color_cast() {
        let ffi_color = crossterm_color {
            t: MAGENTA_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::Magenta);
    }

    #[test]
    fn cyan_color_cast() {
        let ffi_color = crossterm_color {
            t: CYAN_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::Cyan);
    }

    #[test]
    fn grey_color_cast() {
        let ffi_color = crossterm_color {
            t: GREY_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::Grey);
    }

    #[test]
    fn dark_red_color_cast() {
        let ffi_color = crossterm_color {
            t: DARK_RED_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::DarkRed);
    }

    #[test]
    fn dark_green_color_cast() {
        let ffi_color = crossterm_color {
            t: DARK_GREEN_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::DarkGreen);
    }

    #[test]
    fn dark_blue_color_cast() {
        let ffi_color = crossterm_color {
            t: DARK_BLUE_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::DarkBlue);
    }

    #[test]
    fn dark_yellow_color_cast() {
        let ffi_color = crossterm_color {
            t: DARK_YELLOW_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::DarkYellow);
    }

    #[test]
    fn dark_magenta_color_cast() {
        let ffi_color = crossterm_color {
            t: DARK_MAGENTA_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::DarkMagenta);
    }

    #[test]
    fn dark_cyan_color_cast() {
        let ffi_color = crossterm_color {
            t: DARK_CYAN_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::DarkCyan);
    }

    #[test]
    fn dark_grey_color_cast() {
        let ffi_color = crossterm_color {
            t: DARK_GREY_COLOR,
            v: unsafe { std::mem::zeroed() },
        };
        assert_eq!(from_ffi_color_to_rust_color(&ffi_color), Color::DarkGrey);
    }

    #[test]
    fn ansi_color_cast() {
        let ffi_color = crossterm_color {
            t: ANSI_COLOR,
            v: crossterm_color_value {
                ansi: crossterm_ansi_color { value: 59 },
            },
        };
        assert_eq!(
            from_ffi_color_to_rust_color(&ffi_color),
            Color::AnsiValue(59)
        );
    }

    #[test]
    fn rgb_color_cast() {
        let ffi_color = crossterm_color {
            t: RGB_COLOR,
            v: crossterm_color_value {
                rgb: crossterm_rgb_color { r: 5, g: 9, b: 15 },
            },
        };
        assert_eq!(
            from_ffi_color_to_rust_color(&ffi_color),
            Color::Rgb { r: 5, g: 9, b: 15 },
        );
    }
}
