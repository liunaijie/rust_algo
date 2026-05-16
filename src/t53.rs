#[allow(dead_code)]
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut prefix_sum = 0;
    for ele in nums {
        let t = prefix_sum + ele;
        prefix_sum = t.max(ele);
        res = res.max(prefix_sum);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53_1() {
        // Source: 53-最大子数组和.md
        // 示例 1：
        // 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
        // 输出：6
        assert_eq!(6, max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    }

    #[test]
    fn test_53_2() {
        // Source: 53-最大子数组和.md
        // 示例 2：
        // 输入：nums = [1]
        // 输出：1
        assert_eq!(1, max_sub_array(vec![1]));
    }

    #[test]
    fn test_53_3() {
        // Source: 53-最大子数组和.md
        // 示例 3：
        // 输入：nums = [5,4,-1,7,8]
        // 输出：23
        assert_eq!(23, max_sub_array(vec![5, 4, -1, 7, 8]));
    }
}
