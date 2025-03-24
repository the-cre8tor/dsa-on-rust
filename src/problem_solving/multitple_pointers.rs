// Multiple Pointers
// Creating pointers or values that correspond to
// an index or position and move towards the beginning,
// end or middle based on a certain condition.
//
// -> Very efficient for solving problems with
// minimal space complexity as well.

// Space & Time Complexity:
//  - Space: O(1) - uses vector with the capacity of 2 which is known at compiled-time.
//  - Time: O(n) - single pass thruogh the while loop.
pub fn sum_zero(list: &[i8]) -> Vec<i8> {
    let mut left = 0;
    let mut right = list.len() - 1;

    let mut result = Vec::with_capacity(2);

    while left < right {
        let sum = list[left] + list[right];

        if sum == 0 {
            result.push(list[left]);
            result.push(list[right]);
            break;
        } else if sum > 0 {
            right = right - 1;
        } else {
            left = left + 1;
        }
    }

    result
}
