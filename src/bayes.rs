pub fn bayes(b_given_a: f64, a: f64, b_given_not_a: f64, not_a: f64) -> f64 {
    b_given_a * a / (b_given_a * a + b_given_not_a * not_a)
}

#[test]
fn test_bayes() {
    let (cancer, not_cancer) = (0.01, 0.99);
    let (true_pos, false_pos) = (0.8, 0.096);
    let has_cancer = bayes(true_pos, cancer, false_pos, not_cancer);
    assert!((has_cancer - 0.07763975_f64).abs() < 1e-6)
}

