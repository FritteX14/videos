/*
The problem is:
    Given two sorted arrays nums1 and nums2 of size m and n respectively,
    return the median of the two sorted arrays.

    Example 1:
        Input: nums1 = [1,3], nums2 = [2]
        Output: 2.00000
        Explanation: merged array = [1,2,3] and median is 2.

    Example 2:
        Input: nums1 = [1,2], nums2 = [3,4]
        Output: 2.50000
        Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
*/

fn sort(array: &mut [i32]) -> &mut [i32] {
    let slice = array.as_mut();
    slice.sort();

    return array;
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut array = [nums1, nums2].concat();
    let sorted = sort(&mut array);

    let length = sorted.len();
    let mut middle = vec![];

    if length % 2 != 0 {
        middle.push(sorted[length / 2]);
    } else {
        middle.push(sorted[length / 2]);
        middle.push(sorted[(length / 2) - 1]);
    }

    if middle.len() > 1 {
        return (middle[0] + middle[1]) as f64 / 2.0;
    } else {
        return middle[0] as f64;
    }
}

fn main() {
    println!(
        "{:?}",
        find_median_sorted_arrays([1, 3].to_vec(), [2].to_vec())
    );
    println!(
        "{:?}",
        find_median_sorted_arrays([1, 2].to_vec(), [3, 4].to_vec())
    );
}
