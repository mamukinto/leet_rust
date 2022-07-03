pub fn is_palindrome(x: i32) -> bool {
    let mut nx = x;
    let minus = if x < 0 {
        nx = -nx;
        Some("-")
    } else {
        None
    };
    let x_str = format!("{}{}",minus.unwrap_or_default(),nx.to_string());
    let len = x_str.len();
    for (i,c) in x_str.chars().into_iter().enumerate() {
        if x_str.chars().nth(i).unwrap() != x_str.chars().nth(len - i - 1).unwrap() {
            return false
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::is_palindrome;

    #[test]
    fn not_a_palindrome() {
        assert!(!is_palindrome(121234799))
    }

    #[test]
    fn not_a_palindrome_negative() {
        assert!(!is_palindrome(-121))
    }

    #[test]
    fn is_a_palindrome() {
        assert!(is_palindrome(1221))
    }
}
