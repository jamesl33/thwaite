use crate::solver::factorial;

/// comb - TODO
pub const fn combinations(i: usize, r: usize) -> usize {
    if i < r {
        return 0;
    }

    if i == r {
        return 1;
    }

    factorial(i) / (factorial(r) * factorial(i - r))
}
