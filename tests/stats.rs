use learning_rust::stats;

#[test]
fn test_min() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    let minimum = stats::min(&numbers);
    assert_eq!(minimum, Some(1));
}

#[test]
fn test_max() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    let maximum = stats::max(&numbers);
    assert_eq!(maximum, Some(9));

    let max_value = stats::max(&numbers).unwrap();
    assert_eq!(max_value, 9);

    let empty: [i32; 0] = [];
    assert_eq!(stats::max(&empty), None);
    assert_eq!(stats::max(&empty).unwrap_or(0), 0);
}

#[test]
fn test_average() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    let avg = stats::average(&numbers);
    assert_eq!(avg, Some(4.0));

    let real_numbers = [2.5, 3.5, 4.0];
    let avg_real = stats::average(&real_numbers);
    assert!((avg_real.unwrap() - 3.333).abs() < 1e-3);
}

#[test]
fn test_median() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    let med = stats::median(&numbers);
    assert_eq!(med, Some(4.0));

    let even_numbers = [3, 1, 4, 1, 5, 9, 2, 6];
    let med_even = stats::median(&even_numbers);
    assert_eq!(med_even, Some(3.5));
}
