pub fn is_subset_i32(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    let mut contains_counter: i32 = 0;
    for element in first {
        if second.contains(&element) {
            contains_counter += 1;
        }
    }
    contains_counter == (first.len() as i32)
}

#[test]
fn test_is_subset_i32() {
    assert!(is_subset_i32(
        &Vec::from([0, 1, 2, 3]),
        &Vec::from([0, 1, 2, 3])
    ));
    assert!(is_subset_i32(
        &Vec::from([3, 4, 5, 6, 7]),
        &Vec::from([2, 3, 4, 5, 6, 7, 8])
    ));
    assert!(!is_subset_i32(&Vec::from([1, 2, 3]), &Vec::from([0, 1, 2])));
    assert!(!is_subset_i32(
        &Vec::from([2, 3, 4, 5, 6, 7, 8]),
        &Vec::from([3, 4, 5, 6, 7])
    ));
}

pub fn do_overlap_i32(first: &Vec<i32>, second: &[i32]) -> bool {
    for element in first {
        if second.contains(element) {
            return true;
        }
    }
    false
}

#[test]
fn test_do_overlap_i32() {
    assert!(do_overlap_i32(
        &Vec::from([0, 1, 2, 3]),
        &Vec::from([0, 1, 2, 3])
    ));
    assert!(do_overlap_i32(
        &Vec::from([3, 4, 5, 6, 7]),
        &Vec::from([2, 3, 4, 5, 6, 7, 8])
    ));
    assert!(do_overlap_i32(&Vec::from([1, 2, 3]), &Vec::from([0, 1, 2])));
    assert!(do_overlap_i32(
        &Vec::from([2, 3, 4, 5, 6, 7, 8]),
        &Vec::from([3, 4, 5, 6, 7])
    ));
    assert!(!do_overlap_i32(
        &Vec::from([2]),
        &Vec::from([3, 4, 5, 6, 7])
    ));
}

pub fn any_ge(vec: &Vec<i32>, thr: i32) -> bool {
    for n in vec {
        if n >= &thr {
            return true;
        }
    }
    false
}

#[test]
fn test_any_ge() {
    assert!(!any_ge(&[0, 1, 2, 3, 4].to_vec(), 5));
    assert!(any_ge(&[0, 1, 5, 3, 4].to_vec(), 5));
}
