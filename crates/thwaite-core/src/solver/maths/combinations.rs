use crate::solver::factorial;

/// Returns the number of combinations of 'r' sub-pieces within 'i' pieces.
pub const fn combinations(i: usize, r: usize) -> usize {
    if i < r {
        return 0;
    }

    if i == r {
        return 1;
    }

    factorial(i) / (factorial(r) * factorial(i - r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_known_values() {
        assert_eq!(combinations(5, 0), 1);
        assert_eq!(combinations(5, 5), 1);
        assert_eq!(combinations(5, 2), 10);
        assert_eq!(combinations(8, 3), 56);
        assert_eq!(combinations(12, 4), 495);
    }

    #[test]
    fn returns_zero_when_choosing_more_than_available() {
        assert_eq!(combinations(3, 4), 0);
    }

    #[test]
    fn is_symmetric() {
        for i in 0..10 {
            for r in 0..=i {
                assert_eq!(
                    combinations(i, r),
                    combinations(i, i - r),
                    "C({i}, {r}) != C({i}, {})",
                    i - r
                );
            }
        }
    }
}
