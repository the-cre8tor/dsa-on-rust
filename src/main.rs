use dsa::{
    big_o::{
        space_complexity::{double, sum_array},
        time_complexity::{add_up_lite, add_up_to, all_pairs, count_up_and_down, log_at_most_5},
    },
    problem_solving::{
        approach::char_count,
        patterns::{anagram_lite, same_linked, validate_anagram},
    },
};

fn main() {
    let _result = add_up_to(10);
    let _ = add_up_lite(1_000_000_000);
    let __ = count_up_and_down(10);
    let ___ = all_pairs(5);
    let _a = log_at_most_5(20);

    let arr = [1, 2, 3, 4, 5];
    let _b = sum_array(&arr);
    let double = double(&arr);
    println!("double the value: {:?}", double);

    let counter = char_count("Helloo hi!");
    println!("{:?}", counter);

    let value_one = [1, 2, 3, 3];
    let value_two = [4, 1, 9, 9];
    let _find_same = same_linked(&value_one, &value_two);
    let anagram = anagram_lite("hoxxmeu", "ohmexxw");

    println!("Same value: {}", anagram);
}
