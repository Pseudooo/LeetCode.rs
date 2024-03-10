
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; 2];
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            } else if (numbers[i] + numbers[j]) == target {
                result[0] = j as i32;
                result[1] = i as i32;
                break;
            }
        }
    }

    return result;
}

#[cfg(test)]
mod reverse_words_tests {
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
