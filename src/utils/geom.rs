pub fn calc_dist_two_points(p1: (i32, i32), p2: (i32, i32)) -> f32 {
    let x_dist_square: f32 = (p1.0 - p2.0).pow(2) as f32;
    let y_dist_square: f32 = (p1.1 - p2.1).pow(2) as f32;
    (x_dist_square + y_dist_square).powf(0.5)
}

#[test]
fn test_calc_dist_two_points() {
    assert_eq!(calc_dist_two_points((1, 1), (2, 2)), (2 as f32).powf(0.5));
    assert_eq!(calc_dist_two_points((1, 0), (4, 4)), 5 as f32);
}
