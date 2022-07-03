use std::collections::HashMap;
fn main(){}

pub fn roman_to_int(s: String) -> i32 {
    let romant_to_int_mapping = HashMap::from(
        [
            ('I',1),
            // ("IV",4),
            ('V',5),
            // ("IX",9),
            ('X',10),
            // ("XL",40),
            ('L',50),
            // ("XC",90),
            ('C',100),
            // ("CD",400),
            ('D',500),
            // ("CM",900),
            ('M',1000)
        ]
    );
    let mut res = 0;
    let mut skip = false;
    for (i,c) in s.chars().into_iter().enumerate() {
        if i == 0 {
            res += romant_to_int_mapping.get(&c).unwrap().to_owned();
            continue;
        }
        res += match c {
            'V' => {
                if s.chars().nth(i - 1).unwrap() == 'I' {
                    3
                } else {
                    5
                }
            }
            'X' => {
                if s.chars().nth(i - 1).unwrap() == 'I' {
                    8
                } else {
                    10
                }
            }
            'L' => {
                if s.chars().nth(i - 1).unwrap() == 'X' {
                    30
                } else {
                    50
                }
            }
            'C' => {
                if s.chars().nth(i - 1).unwrap() == 'X' {
                    80
                } else {
                    100
                }
            }
            'D' => {
                if s.chars().nth(i - 1).unwrap() == 'C' {
                    300
                } else {
                    500
                }
            }
            'M' => {
                if s.chars().nth(i - 1).unwrap() == 'C' {
                    800
                } else {
                    1000
                }
            }
            _ => {
                romant_to_int_mapping.get(&c).unwrap().to_owned()
            }
        }
        
        
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::roman_to_int;
    #[test]
    fn test1() {
        assert_eq!(roman_to_int("III".to_string()),3)
    }

    #[test]
    fn test2() {
        assert_eq!(roman_to_int("LVIII".to_string()),58)
    }
    #[test]
    fn test3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()),1994)
    }
}
