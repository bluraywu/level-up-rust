mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut curr_char = text.chars().nth(0).unwrap();
        let mut curr_count = 0;
        let mut result = String::new();

        for (idx, c) in text.chars().enumerate() {
            if curr_char != c {
                result.push_str(&*curr_count.to_string());
                result.push(curr_char);
                curr_char = c;
                curr_count = 1;
            } else {
                curr_count += 1;
            }
        }
        result.push_str(&*curr_count.to_string());
        result.push(curr_char);
        result
    }

    pub fn decode(text: &str) -> String {
        let mut digit_count = 0;
        let mut result = String::new();
        for (idx, c) in text.chars().enumerate() {
            if c.is_alphabetic() {
                let count: u32 = text[idx - digit_count..idx].parse().unwrap();
                for _ in 0..count {
                    result.push(c);
                }
                digit_count = 0;
            } else {
                digit_count += 1
            }
        }
        result
    }
}

fn main() {
    // 
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 10A1 20A");
}
