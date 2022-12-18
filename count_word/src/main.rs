fn count_word(s: String, target: String) -> i32 {
    let s = s.to_lowercase();
    let target = target.to_lowercase();

    let mut count = 0;

    for i in 0..s.len() - target.len() + 1 {
        if s[i..i + target.len()] == target {
            count += 1;
        }
    }

    return count;
}

fn main() {
    assert_eq!(
        count_word(
            String::from("She sells seashells by the seashore."),
            String::from("sea")
        ), 2
    );
    assert_eq!(
        count_word(
            String::from("The cat in the hat sat on the mat."),
            String::from("the")
        ), 3
    );
}
