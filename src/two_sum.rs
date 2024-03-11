
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    let mut sorted_head_index = 0;
    let mut sorted_tail_index = sorted_numbers.len() - 1;
    let mut sorted_result = vec![];

    loop {
        let sum = sorted_numbers[sorted_head_index] + sorted_numbers[sorted_tail_index];

        if sum > target {
            sorted_tail_index -= 1;
        } else if sum < target {
            sorted_head_index += 1;
        } else {
            sorted_result.push(sorted_head_index as i32);
            sorted_result.push(sorted_tail_index as i32);
            break;
        }
    }

    let mut result = vec![];
    for i in 0..numbers.len() {
        if numbers[i] == sorted_numbers[sorted_head_index] {
            result.push(i as i32);
        } else if numbers[i] == sorted_numbers[sorted_tail_index] {
            result.push(i as i32);
        }
    }

    return result;
}

#[cfg(test)]
mod two_sum_tests {
    use crate::two_sum::two_sum;

    #[test]
    fn example1() {
        let numbers = [2,7,11,15].iter().cloned().collect();
        let target = 9;
        let expected: Vec<i32> = [0,1].iter().cloned().collect();

        let result = two_sum(numbers, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let numbers = [3,2,4].iter().cloned().collect();
        let target = 6;
        let expected: Vec<i32> = [1,2].iter().cloned().collect();

        let result = two_sum(numbers, target);
        assert_eq!(result, expected)
    }

    #[test]
    fn example3() {
        let numbers = [3,3].iter().cloned().collect();
        let target = 6;
        let expected: Vec<i32> = [0,1].iter().cloned().collect();

        let result = two_sum(numbers, target);
        assert_eq!(result, expected);
    }
}
