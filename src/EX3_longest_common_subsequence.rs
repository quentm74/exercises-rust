// -------------------------------------------------------------------------
// PROBLEM STATEMENT
// build a function that returns the longest common subsequence, only the
// order is important, letters can be ignored
// -------------------------------------------------------------------------

fn longest_common_subsequence(s1: String, s2: String) -> Result<String, LongestCommonSubsequenceErrors> {
    let s1: &Vec<char> = &s1.chars().collect();
    let s2: &Vec<char> = &s2.chars().collect();
    for s1_index in 0..s1.len() {
        let subsequence_length = s1.len() - s1_index;
        let subsequence_to_remove_count = s1.len() - subsequence_length;

        let mut subsequence_to_remove_indexes: Vec<usize> = (0..subsequence_to_remove_count).collect();

        loop {
            let mut subsequence = s1.clone();
            for subsequence_to_remove_index in subsequence_to_remove_indexes.iter().rev() {
                subsequence.remove(*subsequence_to_remove_index);
            }
            match search_for_subsequence(s2, &subsequence) {
                Ok(r) => {
                    if r {
                        return Ok(subsequence.iter().collect());
                    }
                },
                Err(e) => return Err(e),
            }
            match increment_array(&subsequence_to_remove_indexes, s1.len() - 1) {
                Some(v) => subsequence_to_remove_indexes = v,
                None => break,
            }
        }
    };
    Ok("".to_string())
}

fn increment_array(array: &Vec<usize>, max_index: usize) -> Option<Vec<usize>> {
    match array.get(0) {
        Some(v) => if *v == max_index + 1 - array.len() { // incrementation impossible because max value
            return None;
        },
        None => return None, // incrementation impossible because empty array
    }

    let mut array_incremented = array.clone();
    let mut reset_after = array_incremented.len();
    for (i, v) in array_incremented.iter_mut().rev().enumerate() {
        if *v < max_index {
            *v += 1;
            reset_after = i;
            break;
        }
    }
    reset_after = array_incremented.len() - 1 - reset_after; // real index
    if reset_after < array_incremented.len() { // reset all indexes after reset_after
        let mut reset_after_value = array_incremented.get(reset_after)? + 1;
        for to_reset in reset_after + 1.. {
            match array_incremented.get_mut(to_reset) {
                Some(mut v) => {
                    *v = reset_after_value;
                    reset_after_value += 1;
                }
                None => break,
            };
        };
        return Some(array_incremented);
    };
    None
}

fn search_for_subsequence(string: &Vec<char>, subsequence: &Vec<char>) -> Result<bool, LongestCommonSubsequenceErrors> {
    if subsequence.len() > string.len() {
        return Ok(false);
    }
    let mut string_index: usize = 0;
    for (subsequence_index, subsequence_char) in subsequence.iter().enumerate() {
        while match string.get(string_index) {
            Some(v) => *subsequence_char != *v,
            None => false,
        } { string_index += 1; };
        if string_index >= string.len() {
            return Ok(subsequence_index == subsequence.len());
        }
        string_index += 1;
    }
    Ok(true)
}

// -------------------------------------------------------------------------
// ERRORS
// -------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum LongestCommonSubsequenceErrors {}

// -------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_array_1() {
        match increment_array(&vec![0, 1, 3], 4) {
            Some(v) => assert_eq!(v, vec![0, 1, 4]),
            None => assert!(false),
        }
    }

    #[test]
    fn test_increment_array_2() {
        match increment_array(&vec![], 4) {
            Some(v) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_increment_array_3() {
        match increment_array(&vec![3, 4], 4) {
            Some(v) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_longest_common_subsequence_1() {
        match longest_common_subsequence("ABAZDC".to_string(), "BACBAD".to_string()) {
            Ok(v) => assert_eq!(v, "ABAD"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_longest_common_subsequence_2() {
        match longest_common_subsequence("ABAZDCZZ".to_string(), "BACBADZZS".to_string()) {
            Ok(v) => assert_eq!(v, "ABADZZ"),
            Err(_) => assert!(false),
        }
    }
}
