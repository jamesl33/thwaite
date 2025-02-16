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
