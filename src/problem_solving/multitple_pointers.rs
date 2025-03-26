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

// create a variable that records the vector first index pointer
// create a variable that records the vector second index pointer
// create a variable of type vector with an initial value of the function parameter first index
// check it the first index is lower than the first index
//  - if it is, store the value in the second index
//  - move the pointer of the left and right index by one for each
//  -

pub fn count_unique_values(value: &mut Vec<i8>) -> usize {
    if value.len() == 0 {
        return 0;
    }

    let mut left = 0;

    for item in 1..value.len() {
        if value[left] != value[item] {
            left = left + 1;

            // Let me explain the difference between these two operations:
            // 1. value[left] = value[item]
            // 2. value.insert(left, value[item])

            // 1. `value[left] = value[item]`:
            // - This operation replaces the element at index `left` with the element at index `item`
            // - It's a simple assignment/replacement
            // - The array length remains the same
            // - O(1) time complexity
            value[left] = value[item];

            // 2. `value.insert(left, value[item])`:
            // - This operation inserts a new element at index `left`, shifting all subsequent elements to the right
            // - It increases the array length by 1
            // - All elements after `left` need to be moved
            // - O(n) time complexity where n is the number of elements after the insertion point
            // value.insert(left, value[item]);

            // Example:
            // Original array: [1, 2, 3, 4]
            //
            // let left = 1;
            // let item = 3;
            //
            // After value[left] = value[item]:
            // [1, 3, 3, 4]  (2 was replaced by 3)
            //
            // After value.insert(left, value[item]):
            // [1, 3, 2, 3, 4]  (3 was inserted at position 1, everything else shifted right)
        }
    }

    println!("{:?}", value);

    left + 1
}
