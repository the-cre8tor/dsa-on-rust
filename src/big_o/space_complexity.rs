// Space complexity describes the amount of storage
// space an algorithm needs to run as its input grows.

// Auxiliary space complexity:
// The extra or temporary space used by an algorithm,
// EXCLUDING the space taken by input and output.

// O(1) auxiliary space
pub fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0; // Only one extra variable regardless of input size

    for num in arr {
        sum += num;
    }

    sum
}

//  O(n) auxiliary space
pub fn duplicate_array(arr: &[i32]) -> Vec<i32> {
    let mut temp = Vec::with_capacity(arr.len()); // Extra space proportional to input

    for &num in arr {
        temp.push(num);
    }

    temp
}

// O(n) auxiliary space due to recursion stack
pub fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }

    n * factorial(n - 1) // Recursion creates stack frames
}

// O(n) auxiliary space
pub fn double(value: &[i32]) -> Vec<i32> {
    let mut new_array = Vec::new();

    for item in value {
        new_array.push(item * 2);
    }

    new_array
}
