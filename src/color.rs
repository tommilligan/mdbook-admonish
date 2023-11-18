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
        let error = || E::invalid_value(Unexpected::Str(s), &self);

        // remove leading '#', if present
        let s = s.strip_prefix('#').unwrap_or(s);

        if s.len() != 6 {
            return Err(error());
        }

        let parse_hex = |s| u8::from_str_radix(s, 16).map_err(|_| error());

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
    use serde_test::Token::Str;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens};

    const RED: Color = Color::hex(0xFF0000);
    const WHITE: Color = Color::hex(0xFFFFFF);
    const BLACK: Color = Color::hex(0x000000);

    const NUMBERS1: Color = Color::hex(0x012345);
    const NUMBERS2: Color = Color::hex(0x067890);
    const ABCDEF: Color = Color::hex(0xABCDEF);

    #[test]
    fn ser_de_with_hash_upper() {
        assert_tokens(&RED, &[Str("#FF0000")]);
        assert_tokens(&WHITE, &[Str("#FFFFFF")]);
        assert_tokens(&BLACK, &[Str("#000000")]);
        assert_tokens(&NUMBERS1, &[Str("#012345")]);
        assert_tokens(&NUMBERS2, &[Str("#067890")]);
        assert_tokens(&ABCDEF, &[Str("#ABCDEF")]);
    }

    #[test]
    fn de_with_hash_lower() {
        assert_de_tokens(&RED, &[Str("#ff0000")]);
        assert_de_tokens(&WHITE, &[Str("#ffffff")]);
        assert_de_tokens(&BLACK, &[Str("#000000")]);
        assert_de_tokens(&NUMBERS1, &[Str("#012345")]);
        assert_de_tokens(&NUMBERS2, &[Str("#067890")]);
        assert_de_tokens(&ABCDEF, &[Str("#abcdef")]);
    }

    #[test]
    fn de_no_hash_upper() {
        assert_de_tokens(&RED, &[Str("FF0000")]);
        assert_de_tokens(&WHITE, &[Str("FFFFFF")]);
        assert_de_tokens(&BLACK, &[Str("000000")]);
        assert_de_tokens(&NUMBERS1, &[Str("012345")]);
        assert_de_tokens(&NUMBERS2, &[Str("067890")]);
        assert_de_tokens(&ABCDEF, &[Str("ABCDEF")]);
    }

    #[test]
    fn de_no_hash_lower() {
        assert_de_tokens(&RED, &[Str("ff0000")]);
        assert_de_tokens(&WHITE, &[Str("ffffff")]);
        assert_de_tokens(&BLACK, &[Str("000000")]);
        assert_de_tokens(&NUMBERS1, &[Str("012345")]);
        assert_de_tokens(&NUMBERS2, &[Str("067890")]);
        assert_de_tokens(&ABCDEF, &[Str("abcdef")]);
    }

    #[test]
    fn de_errors() {
        assert_de_tokens_error::<Color>(
            &[Str("")],
            "invalid value: string \"\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str(" ")],
            "invalid value: string \" \", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("#")],
            "invalid value: string \"#\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("1")],
            "invalid value: string \"1\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("#1")],
            "invalid value: string \"#1\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("123")],
            "invalid value: string \"123\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("#123")],
            "invalid value: string \"#123\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("#abcde")],
            "invalid value: string \"#abcde\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("#0000000")], // seven 0s
            "invalid value: string \"#0000000\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("#00000")], // five 0s
            "invalid value: string \"#00000\", expected an rgb hex color string",
        );
        assert_de_tokens_error::<Color>(
            &[Str("#abcdeg")],
            "invalid value: string \"#abcdeg\", expected an rgb hex color string",
        );
    }
}
