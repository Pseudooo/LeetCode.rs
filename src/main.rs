use crate::two_sum::two_sum;

mod reverse_words;
mod two_sum;
mod add_two_numbers;
mod product_of_array_except_self;

fn main() {
    let numbers = [3,2,4].iter().cloned().collect();
    let target = 6;
    let expected: Vec<i32> = [1,2].iter().cloned().collect();

    let result = two_sum(numbers, target);
    assert_eq!(result, expected);
}
