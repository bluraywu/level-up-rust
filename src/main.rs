use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum MyError {
    TooLong,
    TooShort,
    FailCheckSum,
}

impl FromStr for Isbn {
    // TODO: replace with appropriate type
    //too long
    //too short
    //failed checksum
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: Vec<u8> = vec![];
        for c in s.chars() {
            if c != '-' {
                digits.push(c.to_digit(10).unwrap() as u8);
            }
        }
        if digits.len() > 13 {
            return Err(MyError::TooLong);
        }
        if digits.len() < 13 {
            return Err(MyError::TooShort);
        }
        if digits[12] != calculate_check_digit(&digits[0..digits.len() - 1]) {
            return Err(MyError::FailCheckSum);
        }
        Ok(
            Self {
                raw: s.to_string(),
                digits,
            }
        )
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut weight: u32 = 0;
    for (idx, digit) in digits.iter().enumerate() {
        weight += if idx % 2 == 0 {
            *digit as u32
        } else {
            (digit * 3) as u32
        };
    }
    ((10 - (weight % 10)) % 10) as u8
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
