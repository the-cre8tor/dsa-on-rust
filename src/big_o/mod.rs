pub fn add_up_to(value: u32) {
    let mut accumulator: u32 = 0;

    for item in 1..=value {
        accumulator += item;
    }

    println!("Result: {}", accumulator);
}
