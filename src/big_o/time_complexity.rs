// Time complexity describes how long an algorithms takes to run as it input grows.

// [time complexity]: linear time - O(n)
pub fn add_up_to(value: u64) {
    let mut accumulator: u64 = 0;

    for item in 1..=value {
        accumulator += item;
    }

    println!("result: {}", accumulator);
}

// [time complexity]: constant time - O(1)
pub fn add_up_lite(value: u64) {
    let accm = value * (value + 1) / 2;
    println!("result: {}", accm);
}

// [time complexity]: linear time - O(n)
pub fn count_up_and_down(value: u64) {
    println!("going up!");

    // O(n) - linear time
    for item in 0..value {
        println!("-> up {}", item)
    }

    println!("at the top!\ngoing down...");

    // O(n) - linear time
    for item in (0..value).rev() {
        println!("<- down {}", item)
    }

    println!("back down. bye!");
}

// [time complexity]: quadratic time - O(n2)
pub fn all_pairs(value: u64) {
    for i in 0..=value {
        for j in 0..=value {
            println!("pairs printer: {},{}", i, j);
        }
    }
}
