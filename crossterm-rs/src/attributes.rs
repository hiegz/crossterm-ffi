pub type crossterm_attributes = u16;

#[rustfmt::skip]
mod unformatted {
    use super::crossterm_attributes;

    pub const CROSSTERM_RESET_ATTRIBUTE:      crossterm_attributes = 0b0000000000000010;
    pub const CROSSTERM_BOLD_ATTRIBUTE:       crossterm_attributes = 0b0000000000000100;
    pub const CROSSTERM_DIM_ATTRIBUTE:        crossterm_attributes = 0b0000000000001000;
    pub const CROSSTERM_UNDERLINED_ATTRIBUTE: crossterm_attributes = 0b0000000000100000;
    pub const CROSSTERM_REVERSE_ATTRIBUTE:    crossterm_attributes = 0b0001000000000000;
    pub const CROSSTERM_HIDDEN_ATTRIBUTE:     crossterm_attributes = 0b0010000000000000;
}

pub use unformatted::*;

pub fn from_ffi_attributes_to_rust_attributes(
    ffi_attributes: crossterm_attributes,
) -> crossterm::style::Attributes {
    unsafe { std::mem::transmute_copy(&(ffi_attributes as u32)) }
}

#[cfg(test)]
mod test {
    use super::*;
    use crossterm::style::Attribute;
    // use crossterm::style::Attributes;

    #[test]
    fn reset_attribute() {
        assert_eq!(CROSSTERM_RESET_ATTRIBUTE as u32, Attribute::Reset.bytes());
    }

    #[test]
    fn has_reset_attribute() {
        let attributes = from_ffi_attributes_to_rust_attributes(CROSSTERM_RESET_ATTRIBUTE);
        assert!(attributes.has(Attribute::Reset));
    }

    #[test]
    fn bold_attribute() {
        assert_eq!(CROSSTERM_BOLD_ATTRIBUTE as u32, Attribute::Bold.bytes());
    }

    #[test]
    fn has_bold_attribute() {
        let attributes = from_ffi_attributes_to_rust_attributes(CROSSTERM_BOLD_ATTRIBUTE);
        assert!(attributes.has(Attribute::Bold));
    }

    #[test]
    fn dim_attribute() {
        assert_eq!(CROSSTERM_DIM_ATTRIBUTE as u32, Attribute::Dim.bytes());
    }

    #[test]
    fn has_dim_attribute() {
        let attributes = from_ffi_attributes_to_rust_attributes(CROSSTERM_DIM_ATTRIBUTE);
        assert!(attributes.has(Attribute::Dim));
    }

    #[test]
    fn underlined_attribute() {
        assert_eq!(
            CROSSTERM_UNDERLINED_ATTRIBUTE as u32,
            Attribute::Underlined.bytes()
        );
    }

    #[test]
    fn has_underlined_attribute() {
        let attributes = from_ffi_attributes_to_rust_attributes(CROSSTERM_UNDERLINED_ATTRIBUTE);
        assert!(attributes.has(Attribute::Underlined));
    }

    #[test]
    fn reverse_attribute() {
        assert_eq!(
            CROSSTERM_REVERSE_ATTRIBUTE as u32,
            Attribute::Reverse.bytes()
        );
    }

    #[test]
    fn has_reverse_attribute() {
        let attributes = from_ffi_attributes_to_rust_attributes(CROSSTERM_REVERSE_ATTRIBUTE);
        assert!(attributes.has(Attribute::Reverse));
    }

    #[test]
    fn hidden_attribute() {
        assert_eq!(CROSSTERM_HIDDEN_ATTRIBUTE as u32, Attribute::Hidden.bytes());
    }

    #[test]
    fn has_hidden_attribute() {
        let attributes = from_ffi_attributes_to_rust_attributes(CROSSTERM_HIDDEN_ATTRIBUTE);
        assert!(attributes.has(Attribute::Hidden));
    }
}
