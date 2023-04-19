fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }
    a.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let length = a.len();
    let middle = length / 2;
    let med = if !is_even_length(length) {
        a[middle]
    } else {
        avg(a[middle - 1], a[middle])
    };
    Some(med)
}

fn avg(a: f32, b: f32) -> f32 {
    (a + b) / 2.0
}

fn is_even_length(len: usize) -> bool {
    len % 2 == 0
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn avg_test() {
    let result = avg(4.0, 4.0);
    assert_eq!(result, 4.0);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
