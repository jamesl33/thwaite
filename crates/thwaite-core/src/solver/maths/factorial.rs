/// An arbitrary number of pre-calculated factorials, increase as required.
const FACTORIALS: [usize; 15] = facts();

/// Calculates the factorials from zero to 'N'.
const fn facts<const N: usize>() -> [usize; N] {
    let mut factorials = [0; N];
    let mut i = 0;

    loop {
        if i == N {
            break;
        }

        // Populate the factorial, where the index maps to the numbers factorial
        factorials[i] = fact(i + 1);

        // Move onto the next integer
        i += 1;
    }

    factorials
}

/// Calculates the factorial of a given number.
const fn fact(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }

    let mut a = 1;
    let mut i = 1;

    loop {
        if i == n {
            break;
        }

        // Multiply the step
        a *= i;

        // Move onto the next step
        i += 1;
    }

    a
}

/// Returns the factorial of the given number, using a pre-calculated table of factorials.
pub const fn factorial(n: usize) -> usize {
    FACTORIALS[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_factorials_of_small_numbers() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn matches_naive_factorial_for_every_precalculated_entry() {
        for n in 0..FACTORIALS.len() {
            let naive: usize = (1..=n).product();

            assert_eq!(factorial(n), naive, "factorial({n}) mismatch");
        }
    }
}
