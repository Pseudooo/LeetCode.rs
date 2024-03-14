
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

    let mut left_products = vec![1; nums.len()];
    let mut right_products = vec![1; nums.len()];
    for idx in 0..nums.len() {
        if idx == 0 {
            left_products[0] = nums[0];
        } else {
            left_products[idx] = nums[idx] * left_products[idx - 1];
        }

        let rev_idx = nums.len() - 1 - idx;
        if rev_idx == nums.len() - 1 {
            right_products[nums.len() - 1] = nums[nums.len() - 1];
        } else {
            right_products[rev_idx] = nums[rev_idx] * right_products[rev_idx + 1];
        }
    }

    let mut results = vec![0; nums.len()];
    for idx in 0..nums.len() {

        let mut left_product = 1;
        if idx > 0 {
            left_product = left_products[idx - 1]
        }

        let mut right_product = 1;
        if idx < nums.len() - 1 {
            right_product = right_products[idx + 1];
        }

        results[idx] = left_product * right_product;
    }

    return results;
}

#[cfg(test)]
mod product_of_array_except_self_tests {
    use crate::product_of_array_except_self::product_except_self;

    #[test]
    fn example1() {
        let input = vec!(1,2,3,4);
        let result = product_except_self(input);

        let expected = vec!(24,12,8,6);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let input = vec!(-1,1,0,-3,3);
        let result = product_except_self(input);

        let expected = vec!(0,0,9,0,0);
        assert_eq!(result, expected);
    }
}