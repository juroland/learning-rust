pub fn min<T>(x: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if x.is_empty() {
        return None;
    }
    let mut min_value = x[0];
    for &value in x.iter() {
        if value < min_value {
            min_value = value;
        }
    }
    Some(min_value)
}

pub fn max<T>(x: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if x.is_empty() {
        return None;
    }
    let mut max_value = x[0];
    for &value in x.iter() {
        if value > max_value {
            max_value = value;
        }
    }
    Some(max_value)
}

pub fn average<T>(x: &[T]) -> Option<f64>
where
    T: num_traits::ToPrimitive + Copy + std::iter::Sum,
{
    if x.is_empty() {
        return None;
    }
    let sum: f64 = x
        .iter()
        .map(|&value| num_traits::ToPrimitive::to_f64(&value).unwrap_or(0.0))
        .sum();
    Some(sum / x.len() as f64)
}

pub fn median<T>(x: &[T]) -> Option<f64>
where
    T: PartialOrd + Copy + num_traits::ToPrimitive,
{
    if x.is_empty() {
        return None;
    }
    let sorted_x = {
        let mut v = x.to_vec();
        v.sort_by(|a, b| {
            a.partial_cmp(b)
                .expect("Elements must be comparable for sorting")
        });
        v
    };
    let len = x.len();
    if len % 2 == 0 {
        let left = num_traits::ToPrimitive::to_f64(&sorted_x[len / 2 - 1])?;
        let right = num_traits::ToPrimitive::to_f64(&sorted_x[len / 2])?;
        Some((left + right) / 2.0)
    } else {
        num_traits::ToPrimitive::to_f64(&sorted_x[len / 2])
    }
}
