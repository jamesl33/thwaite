/// otoidx - TODO
///
/// https://www.jaapsch.net/puzzles/compindx.htm#orient
pub fn otoidx<const N: usize, const O: usize>(orien: &[usize; N]) -> usize {
    let mut dec = 0;

    for i in 0..N - 1 {
        dec += orien[i] * (O as usize).pow(10 - i as u32)
    }

    dec
}
