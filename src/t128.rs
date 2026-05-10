use std::collections::HashSet;

#[allow(dead_code)]
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    // 将每个元素放到set中
    for num in nums {
        set.insert(num);
    }

    let mut longest = 0;
    for num in &set {
        // 注意这里的 ！
        // 这是优化的关键
        // 这里使用 ！，仅会在子序列的开始位置进入循环
        // 否则对于 1，2，3，4这样的序列，会对2，3，4元素进行3次计算，这样则仅会对1计算一次
        if !set.contains(&(num - 1)) {

            let mut current_num = *num;
            let mut current_streak = 1;
            // 由于我们是在自序列的开始位置进入，所以这里需要判断的是 +1 元素
            while set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }
            // 与最大值比较
            longest = longest.max(current_streak)
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_consecutive_1() {
        assert_eq!(longest_consecutive(vec![100,4,200,1,3,2]), 4);
    }
    #[test]
    fn test_longest_consecutive_2() {
        assert_eq!(longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]), 9);
    }
    #[test]
    fn test_longest_consecutive_3() {
        assert_eq!(longest_consecutive(vec![1,0,1,2]), 3);
    }
}