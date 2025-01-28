/// FACTORIAL - TODO
///
/// Arbitrary number, increase as required.
const FACTORIALS: [usize; 15] = facts();

/// facts - TODO
const fn facts<const N: usize>() -> [usize; N] {
    let mut factorials = [0; N];
    let mut i = 0;

    loop {
        if i == N {
            break;
        }

        // TODO
        factorials[i] = fact(i);

        // TODO
        i += 1;
    }

    factorials
}

/// fact - TODO
const fn fact(n: usize) -> usize {
    // TODO
    if n <= 1 {
        return 1;
    }

    let mut a = 1;
    let mut i = 1;

    loop {
        if i == n {
            break;
        }

        // TODO
        a *= i;

        // TODO
        i += 1;
    }

    return a;
}

/// fact - TODO
pub const fn factorial(n: usize) -> usize {
    FACTORIALS[n]
}
