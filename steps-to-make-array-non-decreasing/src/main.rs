fn total_steps(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut steps = 0;
    let mut rm_buffer = Vec::new();

    loop {
        for i in 1..nums.iter().count() {
            if nums[i - 1] > nums[i] {
                rm_buffer.push(i);
            }
        }

        if rm_buffer.iter().count() == 0 {
            break steps;
        }

        for i in 0..rm_buffer.iter().count() {
            nums.remove(rm_buffer[i] - i);
        }

        steps += 1;
        rm_buffer.clear();
    }
}

fn main() {
    assert_eq!(total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]), 3);
    assert_eq!(total_steps(vec![4, 5, 7, 7, 13]), 0);
    assert_eq!(total_steps(vec![]), 0);
}
