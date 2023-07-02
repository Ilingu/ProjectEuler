use num_bigint::BigUint;
use num_traits::FromPrimitive;

pub fn pow(base: usize, exp: u32) -> BigUint {
    BigUint::from_usize(base).unwrap().pow(exp)
}

#[cfg(test)]
mod tests {
    use crate::pow;

    #[test]
    fn problem_16_input() {
        let res = pow(2, 1000)
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .sum::<u32>();
        assert_eq!(res, 1366);
    }
    #[test]
    fn problem_16_baseinput() {
        let res = pow(2, 15)
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .sum::<u32>();
        assert_eq!(res, 26);
    }
}
