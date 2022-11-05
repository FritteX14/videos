fn convert(s: String, num_rows: i32) -> String {
    let s_len = s.chars().count();

    if num_rows == 1 || s_len as i32 <= num_rows {
        return s;
    }

    let mut letters = vec![String::new(); num_rows as usize];
    let mut current_row = 0;
    let mut direction = true;

    for c in s.chars() {
        letters[current_row].push(c);

        if direction {
            current_row += 1;
        } else {
            current_row -= 1;
        }

        if current_row == num_rows as usize - 1 || current_row == 0 {
            direction = !direction;
        }
    }

    return letters.into_iter().collect();
}

fn main() {
    assert_eq!("PAHNAPLSIIGYIR", convert("PAYPALISHIRING".to_string(), 3));
    assert_eq!("PINALSIGYAHRPI", convert("PAYPALISHIRING".to_string(), 4));
}

