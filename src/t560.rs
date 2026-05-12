use std::collections::HashMap;

#[allow(dead_code)]
fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix_sum = vec![0; nums.len() + 1];
    for (index, e) in nums.iter().enumerate() {
        prefix_sum[index + 1] = e + prefix_sum[index];
    }
    let mut cnt = HashMap::with_capacity(prefix_sum.len()); // 预分配空间
    let mut ans = 0;
    for sum in prefix_sum {
        if let Some(&c) = cnt.get(&(sum - k)) {
            ans += c;
        }
        *cnt.entry(sum).or_insert(0) += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_560_1() {
        // Source: 560-和为 K 的子数组.md
        // 示例 1：
        // 输入：nums = [1,1,1], k = 2
        // 输出：2
        assert_eq!(2, subarray_sum(vec![1, 1, 1], 2));
    }

    #[test]
    fn test_560_2() {
        // Source: 560-和为 K 的子数组.md
        // 示例 2：
        // 输入：nums = [1,2,3], k = 3
        // 输出：2
        assert_eq!(2, subarray_sum(vec![1, 2, 3], 3));
    }
}
