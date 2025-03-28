use dsa::{
    big_o::{
        space_complexity::{double, sum_array},
        time_complexity::{add_up_lite, add_up_to, all_pairs, count_up_and_down, log_at_most_5},
    },
    problem_solving::{
        approach::char_count,
        frequency_counter::{anagram_lite, same_linked},
        multitple_pointers::{count_unique_values, sum_zero},
        sliding_windows::max_sub_array_sum,
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
    let _anagram = anagram_lite("hoxxmeu", "ohmexxw");
    let _array = sum_zero(&[-4, -3, -2, -1, 0, 1, 2, 1, 10]);

    let mut vector = Vec::from_iter([1, 2, 2, 5, 7, 7, 99]);
    let _unique = count_unique_values(&mut vector);
    let sliding = max_sub_array_sum(&[2, 6, 9, 2, 1, 8, 5, 6, 3], 3);

    println!("Same value: {:?}", sliding);
}
