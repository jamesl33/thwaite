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
