fn regex(string: &str, pattern: &str) -> Result<bool, ()> {
    return check_regex(&string.chars().collect(), &pattern.chars().collect(), 0, 0);
}

fn check_regex(string: &Vec<char>, pattern: &Vec<char>, string_index: usize, pattern_index: usize) -> Result<bool, ()> {
    if string.len() == string_index && pattern.len() == pattern_index {
        return Ok(true);
    };

    let string_actual_value = match string.get(string_index) {
        Some(v) => *v,
        None => return Ok(false),
    };
    let pattern_actual_value = match pattern.get(pattern_index) {
        Some(v) => *v,
        None => return Ok(false),
    };
    let pattern_next_value = match pattern.get(pattern_index + 1) {
        Some(v) => *v,
        None => return Ok(pattern_actual_value == '.' || pattern_actual_value == string_actual_value),
    };

    if pattern_next_value != '*' {
        if pattern_actual_value == '.' || pattern_actual_value == string_actual_value {
            return check_regex(string, pattern, string_index + 1, pattern_index + 1);
        };
        return Ok(false);
    };

    let mut string_start_index = string_index;
    let mut string_actual_value = string_actual_value;
    while pattern_actual_value == '.' || string_actual_value == pattern_actual_value || string_start_index == string_index {
        match check_regex(string, pattern, string_start_index, pattern_index + 2) {
            Ok(v) => {
                if v {
                    return Ok(true);
                }
            }
            Err(e) => return Err(e),
        };
        string_start_index += 1;
        string_actual_value = match string.get(string_start_index) {
            Some(v) => *v,
            None => {
                match check_regex(string, pattern, string_start_index, pattern_index + 2) {
                    Ok(v) => {
                        if v {
                            return Ok(true);
                        }
                    }
                    Err(e) => return Err(e),
                };
                break;
            },
        };
    };

    return Ok(false);
}

// -------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_1() {
        assert_eq!(regex("aab", "a*."), Ok(true));
    }

    #[test]
    fn test_regex_2() {
        assert_eq!(regex("aacb", "aa.b"), Ok(true));
    }

    #[test]
    fn test_regex_3() {
        assert_eq!(regex("b", "a*."), Ok(true));
    }

    #[test]
    fn test_regex_4() {
        assert_eq!(regex("aacb", "aa.c"), Ok(false));
    }

    #[test]
    fn test_regex_5() {
        assert_eq!(regex("aacbhbhjebhehbejeb", ".*"), Ok(true));
    }

    #[test]
    fn test_regex_6() {
        assert_eq!(regex("abb", "ab*"), Ok(true));
    }

    #[test]
    fn test_regex_7() {
        assert_eq!(regex("abbbbbbbc", "ab*"), Ok(false));
    }

    #[test]
    fn test_regex_8() {
        assert_eq!(regex("abbbbbbb", "ab*c"), Ok(false));
    }
}