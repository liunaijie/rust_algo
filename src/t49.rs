use std::collections::BTreeMap;

#[allow(dead_code)]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // 使用BTreeMap，因为BTreeMap是有序的，所以可以保证结果是有序的
    // HashMap每次使用RandomState,每次的hash种子不一致，所以结果不一致
    let mut map = BTreeMap::new();
    for str in strs {
        // 将字符串转换为字节数组并排序，排序后的内容如果一样，则为字母异位词
        let mut sorted = str.as_bytes().to_vec();
        sorted.sort_unstable();
        // 在map中查找排序后相同的字符串
        // 如果找到，则将字符串添加到对应的vec中
        // 如果没找到，则创建新的vec并添加到map中
        map.entry(sorted).or_insert(vec![]).push(str);
    }
    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_anagrams_1() {
        let strs = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
        let result = group_anagrams(strs);
        assert_eq!(result, 
            vec![
                vec!["bat".to_string()], 
                vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
                vec!["tan".to_string(), "nat".to_string()] 
            ]
        );
    }

    #[test]
    fn test_group_anagrams_2() {
        let strs = vec!["".to_string()];
        let result = group_anagrams(strs);
        assert_eq!(result, vec![vec!["".to_string()]]);
    }
    #[test]
    fn test_group_anagrams_3() {
        let strs = vec!["a".to_string()];
        let result = group_anagrams(strs);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }
}