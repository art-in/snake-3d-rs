use crate::models::Range;

/// Normalizes any number to an arbitrary range
/// by assuming the range wraps around when going below min or above max
pub fn normalize_by_circular_range(value: f64, start: f64, end: f64) -> f64 {
    let width = end - start;
    let offset_value = value - start; // value relative to 0

    offset_value - (offset_value / width).floor() * width + start
    // + start to reset back to start of original range
}

pub fn project_to_range(a: f64, range_a: Range, range_b: Range) -> f64 {
    let da = range_a.1 - range_a.0;
    let dn = a - range_a.0;
    let ratio = dn / da;

    let db = range_b.1 - range_b.0;

    range_b.0 + db * ratio
}

#[cfg(test)]
mod tests {
    use super::*;

    mod normalize_by_circular_range {
        use super::*;

        #[test]
        fn basic() {
            assert_eq!(normalize_by_circular_range(0.0, 0.0, 10.0), 0.0);
            assert_eq!(normalize_by_circular_range(5.0, 0.0, 10.0), 5.0);
            assert_eq!(normalize_by_circular_range(10.0, 0.0, 10.0), 0.0);
            assert_eq!(normalize_by_circular_range(11.0, 0.0, 10.0), 1.0);
            assert_eq!(normalize_by_circular_range(-1.0, 0.0, 10.0), 9.0);
        }
    }

    mod project_to_range {
        use super::*;

        #[test]
        fn basic() {
            assert_eq!(
                project_to_range(5.0, Range(0.0, 10.0), Range(10.0, 15.0)),
                12.5
            );
        }
    }
}
