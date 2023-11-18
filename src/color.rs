use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display, Formatter};

// TODO is there a sufficient lib for this type?
/// An RGB color
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub(crate) const fn hex(hex: u32) -> Self {
        assert!(hex <= 0xFFFFFF, "color out of range");

        let red = (hex >> 16) as u8;
        let green = (hex >> 8) as u8;
        let blue = hex as u8;

        Color { red, green, blue }
    }

    #[allow(unused)]
    pub(crate) const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // the fmt specifier `:02X` will print out the int as 2 digit uppercase hex

        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("an rgb hex color string")
    }

    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
        // remove leading '#', if present
        let s = s.strip_prefix('#').unwrap_or(s);

        if s.len() != 6 {
            return Err(E::invalid_value(Unexpected::Str(s), &self));
        }

        let parse_hex =
            |s| u8::from_str_radix(s, 16).map_err(|_| E::invalid_value(Unexpected::Str(s), &self));

        let red = parse_hex(&s[0..2])?;
        let green = parse_hex(&s[2..4])?;
        let blue = parse_hex(&s[4..6])?;

        Ok(Color { red, green, blue })
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_str(ColorVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::assert_tokens;
    use serde_test::Token::Str;

    const RED: Color = Color::rgb(255, 0, 0);
    const WHITE: Color = Color::rgb(255, 255, 255);
    const BLACK: Color = Color::rgb(0, 0, 0);
    // TODO more colors to test with

    #[test]
    fn hex_constructor() {
        assert_eq!(Color::hex(0xFF0000), RED);
        assert_eq!(Color::hex(0xFFFFFF), WHITE);
        assert_eq!(Color::hex(0x000000), BLACK);
    }

    #[test]
    fn ser_de_with_hash() {
        assert_tokens(&RED, &[Str("#FF0000")]);
        assert_tokens(&WHITE, &[Str("#FFFFFF")]);
        assert_tokens(&BLACK, &[Str("#000000")]);
    }

    // TODO more tests
}
