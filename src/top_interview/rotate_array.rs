/// # Rotate Array
/// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
///
/// ## Example 1:
/// ```text
/// Input: nums = [1,2,3,4,5,6,7], k = 3
/// Output: [5,6,7,1,2,3,4]
/// Explanation:
/// rotate 1 steps to the right: [7,1,2,3,4,5,6]
/// rotate 2 steps to the right: [6,7,1,2,3,4,5]
/// rotate 3 steps to the right: [5,6,7,1,2,3,4]
/// ```
///
/// ## Example 2:
/// ```text
/// Input: nums = [-1,-100,3,99], k = 2
/// Output: [3,99,-1,-100]
/// Explanation:
/// rotate 1 steps to the right: [99,-1,-100,3]
/// rotate 2 steps to the right: [3,99,-1,-100]
/// ```
///
/// ## Constraints:
/// * `1 <= nums.length <= 105`
/// * `-231 <= nums[i] <= 231 - 1`
/// * `0 <= k <= 105`
///
/// ## Follow up:
/// * Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
/// * Could you do it in-place with O(1) extra space?
pub struct Solutions;

impl Solutions {
    pub fn built_in_rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.rotate_right(k);
    }
}

#[cfg(test)]
mod rotate_array_tests {
    use super::*;

    macro_rules! test_case {
        ($nums: expr, $k: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let mut nums: Vec<i32> = $nums;
                let k: i32 = $k;
                let expected: Vec<i32> = $expected;

                Solutions::built_in_rotate(&mut nums, k);
                assert_eq!(nums, expected);
            }
        };
        (
            nums=$nums: expr,
            k=$k: expr,
            expected=$expected: expr,
            test_case_name=$fn_name: ident
        ) => {
            test_case!($nums, $k, $expected, $fn_name);
        };
    }

    test_case!(
        nums = vec![1, 2, 3, 4, 5, 6, 7],
        k = 3,
        expected = vec![5, 6, 7, 1, 2, 3, 4],
        test_case_name = test_case_1
    );
    test_case!(vec![1], 13, vec![1], test_case_2);
    test_case!(vec![1, 2, 3, 4], 25, vec![4, 1, 2, 3], test_case_3);
    test_case!(vec![-100, 99, 13, 29], 0, vec![-100, 99, 13, 29], test_case_4);
    test_case!(vec![5, 6, 7, 8, 9], 105, vec![5, 6, 7, 8, 9], test_case_5);
    test_case!(vec![5, 6, 7, 8], 105, vec![8, 5, 6, 7], test_case_6);
}
