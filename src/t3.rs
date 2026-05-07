use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut ans = 0;
    let mut left = 0;
    let mut h = HashMap::new();
    for (right, c) in s.chars().enumerate() {
        // 如果c之前存在，则当前位置的 无重复元素的字串 应该从 i+1位置开始
        if let Some(&prev_index) = h.get(&c) {
            left = left.max(prev_index + 1);
        }
        // 计算当前字串的长度，与结果进行比较
        ans = ans.max(right - left + 1);
        // 更新当前元素的位置，用于判断是否重复以及获取位置
        h.insert(c, right);
        
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_1() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
    }

    #[test]
    fn test_3_2() {
        assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
    }

    #[test]
    fn test_3_3() {
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
    }
}
