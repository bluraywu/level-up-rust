use std::fmt::Display;
use std::str::FromStr;
use std::u8;

#[derive(Debug, PartialEq)]
struct Rgb {
    raw: String,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        u8::from_str_radix(&self.raw[1..3], 16).unwrap()
    }

    fn g(&self) -> u8 {
        u8::from_str_radix(&self.raw[3..5], 16).unwrap()
    }

    fn b(&self) -> u8 {
        u8::from_str_radix(&self.raw[5..], 16).unwrap()
    }
}

#[derive(Debug)]
enum MyErr {
    TooLong,
    TooShort,
    NotStartWithHash,
    OutOfBounds,
    InvalidLiterals,
}

impl FromStr for Rgb {
    type Err = MyErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MyErr::*;
        if s.len() > 7 {
            return Err(TooLong);
        }
        if s.len() < 7 {
            return Err(TooShort);
        }
        if !s.starts_with("#") {
            return Err(NotStartWithHash);
        }
        for idx in (1..s.len()).step_by(2) {
            for x in s[idx..idx + 2].chars() {
                if !x.is_digit(16) {
                    return Err(InvalidLiterals);
                }
            }
            if u8::from_str_radix(&s[idx..idx + 2], 16).is_err() {
                return Err(OutOfBounds);
            }
        }
        Ok(Rgb { raw: s.to_string() })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    // 
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}

