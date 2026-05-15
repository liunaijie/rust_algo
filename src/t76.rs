#[allow(dead_code)]
fn min_window(s: String, t: String) -> String {
    fn is_covered(cnt_s: &[i32; 128], cnt_t: &[i32; 128]) -> bool {
        for i in b'A'..=b'Z' {
            if cnt_s[i as usize] < cnt_t[i as usize] {
                return false;
            }
        }
        for i in b'a'..=b'z' {
            if cnt_s[i as usize] < cnt_t[i as usize] {
                return false;
            }
        }
        true
    }

    let mut cnt_s = [0; 128]; // s 子串字母的出现次数
    let mut cnt_t = [0; 128]; // t 中字母的出现次数
    for c in t.bytes() {
        cnt_t[c as usize] += 1;
    }

    let s_bytes = s.as_bytes();
    let m = s_bytes.len();
    let mut ans_left = 0;
    let mut ans_right = m;
    let mut left = 0;

    let mut ans = "";

    for (right, &c) in s_bytes.iter().enumerate() {
        // 移动子串右端点
        cnt_s[c as usize] += 1; // 右端点字母移入子串
        while is_covered(&cnt_s, &cnt_t) {
            // 涵盖
            if right - left < ans_right - ans_left {
                // 找到更短的子串
                ans_left = left; // 记录此时的左右端点
                ans_right = right;
                // 更新结果
                ans = &s[ans_left..=ans_right];
            }
            // 左端点字母移出子串
            cnt_s[s_bytes[left] as usize] -= 1; 
            
            left += 1;
        }
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76_1() {
        // Source: 76-最小覆盖子串.md
        // 示例 1：
        // 输入：s = "ADOBECODEBANC", t = "ABC"
        // 输出："BANC"
        assert_eq!(
            "BANC",
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
        );
    }

    #[test]
    fn test_76_2() {
        // Source: 76-最小覆盖子串.md
        // 示例 2：
        // 输入：s = "a", t = "a"
        // 输出："a"
        assert_eq!("a", min_window("a".to_string(), "a".to_string()));
    }

    #[test]
    fn test_76_3() {
        // Source: 76-最小覆盖子串.md
        // 示例 3：
        // 输入: s = "a", t = "aa"
        // 输出: ""
        assert_eq!("", min_window("a".to_string(), "aa".to_string()));
    }
}
