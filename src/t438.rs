#[allow(dead_code)]
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    // 统计p中每个字符出现的次数
    let mut cnt_p = [0; 26];
    for c in p.bytes() {
        cnt_p[(c - b'a') as usize] += 1;
    }
    let chars = s.as_bytes();
    let mut ans = vec![];
    let mut cnt_s = [0; 26];
    for (right, &c) in chars.iter().enumerate() {
        cnt_s[(c - b'a') as usize] += 1;
        // 这里注意，usize的操作，不能为0
        if right + 1 < p.len() {
            continue;
        }
        let left = right + 1 - p.len();
        if cnt_s == cnt_p {
            ans.push(left as i32);
        }
        cnt_s[(chars[left] - b'a') as usize] -= 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_438_1() {
        assert_eq!(
            vec![0, 6],
            find_anagrams("cbaebabacd".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn test_438_2() {
        assert_eq!(
            vec![0, 1, 2],
            find_anagrams("abab".to_string(), "ab".to_string())
        );
    }
}
