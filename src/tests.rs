#[cfg(test)]
mod tests {

    extern crate test;
    use test::Bencher;
    use crate::board::Board;
    use crate::{board_combinations_recursive};

    #[bench]
    fn bench_small_board(b: &mut Bencher) {
        let mut counter: i64 = 0;
        b.iter(|| board_combinations_recursive(Board::new([4,3]), &mut counter));
    }

    #[test]
    fn check_win_test() {
        let val = 1;
        assert_eq!(1, val)
    }

    #[bench]
    fn bit_bench(b: &mut Bencher) {
        b.iter(|| {
            let mut x: u64 = 0;
            for i in 0..u64::MAX {
                x += 1;
            }
        });
    }
}






