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

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns every permutation of `[0, 1, 2, 3]`, via Heap's algorithm.
    fn permutations_of_four() -> Vec<[u8; 4]> {
        fn recurse(arr: &mut [u8; 4], k: usize, out: &mut Vec<[u8; 4]>) {
            if k == arr.len() {
                out.push(*arr);
                return;
            }

            for i in k..arr.len() {
                arr.swap(k, i);
                recurse(arr, k + 1, out);
                arr.swap(k, i);
            }
        }

        let mut out = Vec::new();
        recurse(&mut [0, 1, 2, 3], 0, &mut out);

        out
    }

    #[test]
    fn ptoidx_and_idxtoperm_are_inverses_for_every_permutation_of_four() {
        for perm in permutations_of_four() {
            let idx = ptoidx(&perm);

            assert_eq!(idxtoperm::<4>(idx), perm, "idxtoperm(ptoidx({perm:?})) mismatch");
        }
    }

    #[test]
    fn ptoidx_produces_distinct_indices_for_every_permutation_of_four() {
        let mut indices: Vec<usize> = permutations_of_four().iter().map(ptoidx).collect();
        indices.sort();
        indices.dedup();

        assert_eq!(indices.len(), 24);
    }

    #[test]
    fn invert_undoes_a_permutation() {
        let perm: [u8; 4] = [2, 0, 3, 1];
        let inv = invert(&perm);

        for i in 0..4 {
            assert_eq!(inv[perm[i] as usize], i as u8);
        }
    }

    #[test]
    fn invert_of_identity_is_identity() {
        let identity: [u8; 5] = [0, 1, 2, 3, 4];

        assert_eq!(invert(&identity), identity);
    }
}
