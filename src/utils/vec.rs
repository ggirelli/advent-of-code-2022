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
    assert_eq!(
        is_subset_i32(&Vec::from([0, 1, 2, 3]), &Vec::from([0, 1, 2, 3])),
        true
    );
    assert_eq!(
        is_subset_i32(
            &Vec::from([3, 4, 5, 6, 7]),
            &Vec::from([2, 3, 4, 5, 6, 7, 8])
        ),
        true
    );
    assert_eq!(
        is_subset_i32(&Vec::from([1, 2, 3]), &Vec::from([0, 1, 2])),
        false
    );
    assert_eq!(
        is_subset_i32(
            &Vec::from([2, 3, 4, 5, 6, 7, 8]),
            &Vec::from([3, 4, 5, 6, 7])
        ),
        false
    );
}

pub fn do_overlap_i32(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    for element in first {
        if second.contains(&element) {
            return true;
        }
    }
    false
}

#[test]
fn test_do_overlap_i32() {
    assert_eq!(
        do_overlap_i32(&Vec::from([0, 1, 2, 3]), &Vec::from([0, 1, 2, 3])),
        true
    );
    assert_eq!(
        do_overlap_i32(
            &Vec::from([3, 4, 5, 6, 7]),
            &Vec::from([2, 3, 4, 5, 6, 7, 8])
        ),
        true
    );
    assert_eq!(
        do_overlap_i32(&Vec::from([1, 2, 3]), &Vec::from([0, 1, 2])),
        true
    );
    assert_eq!(
        do_overlap_i32(
            &Vec::from([2, 3, 4, 5, 6, 7, 8]),
            &Vec::from([3, 4, 5, 6, 7])
        ),
        true
    );
    assert_eq!(
        do_overlap_i32(&Vec::from([2]), &Vec::from([3, 4, 5, 6, 7])),
        false
    );
}
