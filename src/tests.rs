#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    #[bench]
    fn game_bench(b: &mut Bencher) {
        b.iter(|| {
            let mut vec = Vec::with_capacity(100);
            for i in 0..100 {
                vec.push(i);
            }
        });
    }

    #[test]
    fn check_win_test() {
        let val = 1;
        assert_eq!(1, val)
    }
}






