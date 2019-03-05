// -------------------------------------------------------------------------
// PROBLEM STATEMENT
// build a function that perform a quicksort on a vec of i32
// -------------------------------------------------------------------------

use std::error::Error;
use std::fmt;

fn quicksort(array: &mut Vec<i32>) -> Result<(), QuicksortErrors> {
    let len = array.len();
    quicksort_r(array, 0, len - 1)
}

fn quicksort_r(array: &mut Vec<i32>, min: usize, max: usize) -> Result<(), QuicksortErrors> {
    if min < max {
        let partition = match quicksort_partition(array, min, max) {
            Ok(tmp) => match tmp {
                Some(v) => v,
                None => return Err(QuicksortErrors::IndexOutOfBoundError),
            },
            Err(e) => return Err(e),
        };
        match quicksort_r(array, min, partition - 1) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match quicksort_r(array, partition, max) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn quicksort_partition(array: &mut Vec<i32>, min: usize, max: usize) -> Result<Option<usize>, QuicksortErrors> {
    let mut min = min;
    if min < max {
        let pivot = match array.get(max) {
            Some(v) => *v,
            None => return Err(QuicksortErrors::IndexOutOfBoundError),
        };
        for i in min..max {
            let value_i = match array.get(i) {
                Some(v) => *v,
                None => return Err(QuicksortErrors::IndexOutOfBoundError),
            };
            if value_i < pivot {
                match swap(array, i, min) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                min += 1;
            }
        }
        match swap(array, min, max) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(Some(min))
}

fn swap(array: &mut Vec<i32>, i: usize, j: usize) -> Result<(), QuicksortErrors> {
    if i != j {
        let value_i = match array.get(i) {
            Some(v) => *v,
            None => return Err(QuicksortErrors::IndexOutOfBoundError),
        };
        let value_j = match array.get(j) {
            Some(v) => *v,
            None => return Err(QuicksortErrors::IndexOutOfBoundError),
        };
        match array.get_mut(i) {
            Some(v) => *v = value_j,
            None => return Err(QuicksortErrors::IndexOutOfBoundError),
        }
        match array.get_mut(j) {
            Some(v) => *v = value_i,
            None => return Err(QuicksortErrors::IndexOutOfBoundError),
        }
    }
    Ok(())
}

// -------------------------------------------------------------------------
// ERRORS
// -------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum QuicksortErrors {
    IndexOutOfBoundError,
}

#[derive(Debug)]
struct IndexOutOfBoundError {}

impl Error for IndexOutOfBoundError {
    fn description(&self) -> &str {
        "Index out of bound"
    }
}

impl fmt::Display for IndexOutOfBoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index out of bound")
    }
}

// -------------------------------------------------------------------------
// TESTS
// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort_1() {
        let mut v = vec![0, 1, 3, 4, 2];
        match quicksort(&mut v) {
            Ok(_) => assert_eq!(v, vec![0, 1, 2, 3, 4]),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_quicksort_2() {
        let mut v = vec![0, 1, 6, 7, 5, 3, 8, 4, 9, 2];
        match quicksort(&mut v) {
            Ok(_) => assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_quicksort_partition_1() {
        let mut v = vec![0, 1, 3, 4, 2];
        match quicksort_partition(&mut v, 0, 4) {
            Ok(tmp) => match tmp {
                Some(r) => {
                    assert_eq!(r, 2);
                }
                None => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_quicksort_partition_2() {
        let mut v = vec![0, 1, 3, 4, 2];
        match quicksort_partition(&mut v, 0, 4) {
            Ok(tmp) => match tmp {
                Some(_) => {
                    assert_eq!(v, vec![0, 1, 2, 4, 3]);
                }
                None => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_swap_1() {
        let mut v = vec![0, 1, 3, 4, 2];
        match swap(&mut v, 1, 2) {
            Ok(_) => assert_eq!(v, vec![0, 3, 1, 4, 2]),
            Err(_) => assert!(false),
        }
    }
}
