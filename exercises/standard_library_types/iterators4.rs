// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    (2..=num).fold(1, |mut accum, val| accum * val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }
    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn factorial_of_0_through_20() {
        for n in 1..=20u64 {
            let mut fact: u64 = 1;
            for i in 1..=n {
                fact *= i
            }
            assert_eq!(fact, factorial(n));
        }
    }

    #[test]
    #[should_panic()]
    fn factorial_of_21_panics() {
        assert_eq!(0, factorial(21));
    }
}
