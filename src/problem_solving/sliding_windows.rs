/**
 * SLIDING WINDOW:
 * This pattern involves creating a window which can either be an
 * array or number from one position to another. Depending on a certain
 * condition, the window increases or closes (and a new window is created).
 *
 * Very useful for keeping track of a subset of data in an array/string.
 */

pub fn max_sub_array_sum(list: &[i32], number: usize) -> i32 {
    if list.len() < number {
        return 0;
    }

    let mut max_sum = 0;

    for item in 0..number {
        max_sum = max_sum + list[item];
    }

    let mut temp_sum = max_sum;

    for item in number..list.len() {
        temp_sum = temp_sum - list[item - number] + list[item];
        max_sum = max_sum.max(temp_sum);
    }

    max_sum
}
