use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::i32::MAX;

fn get_third_minimal_value(set: HashSet<i32>) -> Result<i32, GetThirdMinimalValueErrors> {
    if set.len() >= 3 {} else {
        return Err(GetThirdMinimalValueErrors::SetMustContainAtLeastThreeValuesError);
    }
    let mut minimal_values: Vec<i32> = vec![MAX, MAX, MAX];
    for x in set {
        insert_minimal_value(&mut minimal_values, x);
    }
    return match minimal_values.get(2) {
        Some(v) => Ok(*v),
        None => Err(GetThirdMinimalValueErrors::SetMustContainAtLeastThreeValuesError)
    };
}

fn insert_minimal_value(minimal_values: &mut Vec<i32>, value: i32) {
    let mut insert_at_index: Option<usize> = None;
    for (i, minimal_value) in minimal_values.iter().enumerate() {
        if value < *minimal_value {
            insert_at_index = Some(i);
            break;
        }
    }
    match insert_at_index {
        Some(i) => {
            minimal_values.insert(i, value);
            minimal_values.pop();
        },
        None => {}
    }
}

// -------------------------------------------------------------------------
// ERRORS
// -------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum GetThirdMinimalValueErrors {
    SetMustContainAtLeastThreeValuesError,
}

#[derive(Debug)]
struct SetMustContainAtLeastThreeValuesError {}

impl Error for SetMustContainAtLeastThreeValuesError {
    fn description(&self) -> &str {
        "Set must contain at least three values"
    }
}

impl fmt::Display for SetMustContainAtLeastThreeValuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Set must contain at least three values")
    }
}

// -------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_minimal_value_1() {
        match get_third_minimal_value([1, 3, 2, 6, 8].iter().cloned().collect()) {
            Ok(v) => assert_eq!(v, 3),
            Err(e) => assert!(false),
        }
    }

    #[test]
    fn test_third_minimal_value_2() {
        match get_third_minimal_value([1, 3].iter().cloned().collect()) {
            Ok(v) => assert!(false),
            Err(e) => assert_eq!(e, GetThirdMinimalValueErrors::SetMustContainAtLeastThreeValuesError),
        }
    }
}