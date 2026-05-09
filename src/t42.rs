#[allow(dead_code)]
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    }
    let mut left_max = vec![0; n];
    left_max[0] = height[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(height[i]);
    }

    let mut right_max = vec![0; n];
    right_max[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(height[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        ans += left_max[i].min(right_max[i]) - height[i];
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42_1() {
        // Source: 42-接雨水.md
        // 示例 1：
        // 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
        // 输出：6
        assert_eq!(6, trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }

    #[test]
    fn test_42_2() {
        // Source: 42-接雨水.md
        // 示例 2：
        // 输入：height = [4,2,0,3,2,5]
        // 输出：9
        assert_eq!(9, trap(vec![4, 2, 0, 3, 2, 5]));
    }
}
