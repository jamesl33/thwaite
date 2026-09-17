/// Returns the index within the pruning table for the given permutation, ranked as its Lehmer code amongst all
/// `N!` orderings.
///
/// https://www.jaapsch.net/puzzles/compindx.htm#perm
pub fn ptoidx<const N: usize>(perms: &[u8; N]) -> usize {
    let mut t = 0;

    for i in 0..N - 1 {
        t *= N - i;

        for j in i + 1..N {
            if perms[i] > perms[j] {
                t += 1;
            }
        }
    }

    t
}

/// Returns the permutation of length `N`, ranked at Lehmer-code index `idx` amongst all `N!` orderings; the
/// inverse of `ptoidx`.
pub fn idxtoperm<const N: usize>(mut idx: usize) -> [u8; N] {
    let mut digits = [0usize; N];

    for i in (0..N - 1).rev() {
        let radix = N - i;
        digits[i] = idx % radix;
        idx /= radix;
    }

    let mut remaining: Vec<usize> = (0..N).collect();
    let mut perm = [0u8; N];

    for (i, d) in digits.iter().enumerate() {
        perm[i] = remaining.remove(*d) as u8;
    }

    perm
}

/// Returns the inverse of the given permutation, i.e. the permutation `inv` such that `inv[perm[i]] == i`.
pub fn invert<const N: usize>(perm: &[u8; N]) -> [u8; N] {
    let mut inv = [0u8; N];

    for (i, p) in perm.iter().enumerate() {
        inv[*p as usize] = i as u8;
    }

    inv
}
