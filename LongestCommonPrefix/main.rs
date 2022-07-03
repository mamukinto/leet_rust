fn main() {}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].to_owned();
    }
    let mut are_words_equal = true;
    for s in strs.iter() {
        if *s != strs[0] {
            are_words_equal = false;
        }
    }
    if are_words_equal {
        return strs[0].to_owned();
    }
    let mut common_char_vec: Vec<char> = Vec::new();
    let mut common_index = 0;
    let mut flag = true;
    while (flag) {
        for (i,s) in strs.iter().enumerate() {
            if s.len() == common_index {
                if common_char_vec.len() != common_index {
                    common_char_vec.pop();
                }
                return common_char_vec.into_iter().collect();
            }
            if common_char_vec.len() == common_index && s.len() >= common_index {
                common_char_vec.push(s.chars().nth(common_index).unwrap())
            } else if (common_char_vec[common_index] != s.chars().nth(common_index).unwrap()) {
                flag = false;
            }
        }
        common_index += 1;
    }
    common_char_vec.pop();
    common_char_vec.into_iter().collect()
}

#[cfg(test)]
mod test {
    use crate::longest_common_prefix;

    #[test]
    fn finds_fl_prefix() {
        let strs = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
        assert_eq!(longest_common_prefix(strs),"fl".to_string())
    }

    #[test]
    fn no_common_prefix() {
        let strs = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
        assert_eq!(longest_common_prefix(strs),"".to_string())
    }
    #[test]
    fn one_empty_string() {
        let strs = vec!["".to_string()];
        assert_eq!(longest_common_prefix(strs),"".to_string())
    }
    #[test]
    fn one_unit_size_string() {
        let strs = vec!["".to_string()];
        assert_eq!(longest_common_prefix(strs),"".to_string())
    }
    #[test]
    fn another_test() {
        let strs = vec!["ab".to_string(), "a".to_string()];
        assert_eq!(longest_common_prefix(strs),"a".to_string())
    }

    #[test]
    fn single_word() {
        let strs = vec!["abc".to_string()];
        assert_eq!(longest_common_prefix(strs),"abc".to_string())
    }

    #[test]
    fn multiple_equal_words() {
        let strs = vec!["flower".to_string(),"flower".to_string(),"flower".to_string(),"flower".to_string()];
        assert_eq!(longest_common_prefix(strs),"flower".to_string())
    }

    #[test]
    fn another_another_test() {
        let strs = vec!["a".to_string(),"ac".to_string()];
        assert_eq!(longest_common_prefix(strs),"a".to_string())
    }
}
