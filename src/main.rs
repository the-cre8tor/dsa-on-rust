use dsa::big_o::time_complexity::{add_up_lite, add_up_to, all_pairs, count_up_and_down};

fn main() {
    let _result = add_up_to(1_000_000_000);
    let _ = add_up_lite(1_000_000_000);
    let __ = count_up_and_down(10);
    let ___ = all_pairs(5);
}
