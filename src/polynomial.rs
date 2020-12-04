use num_bigint::{BigUint, RandBigInt};
use num_traits::pow::Pow;
use std::clone::Clone;
use std::ops::*;
use std::vec::Vec;

/// Shamir Secret Shares on Polynomial, (k,n) threshhod secheme
/// P(x) = a_0*x^0 + a_1*x^1 + a_2*x^2 + ... + a_n*x^(k-1)
/// degree is k - 1
/// s = P(0) = a_0
/// s_i = P(i)
pub struct Polynomial {
    pub coefficients: Vec<BigUint>,
}

impl Polynomial {
    /// Create empty Polynomial instance
    pub fn new() -> Self {
        return Polynomial {
            coefficients: Vec::new(),
        };
    }

    pub fn init_coefficients(&mut self, coefficients: Vec<BigUint>) {
        self.coefficients = coefficients;
    }

    pub fn init(&mut self, degree: i32, q: BigUint) {
        let mut coefficients = vec![];
        let mut rng = rand::thread_rng();
        // [0,degree] not [0,degree)
        for _ in 0..=degree {
            let coefficient = rng.gen_biguint_below(&q);
            coefficients.push(coefficient);
        }

        self.init_coefficients(coefficients);
    }

    /// Get P(x) = value
    pub fn get_value(&self, x: BigUint) -> BigUint {
        // a_0
        let mut result = self.coefficients[0].clone();
        // a0+ a_1*x^1 + a_2*x^2 + ... + a_n*x^n
        for i in 1..self.coefficients.len() {
            result = result + self.coefficients[i].clone().mul(x.pow(i));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_init_polynomial() {
        use super::BigUint;
        use super::Polynomial;

        let mut polynomial = Polynomial::new();
        let degree = 3;
        polynomial.init(degree, BigUint::from(5_u32));

        assert_eq!(polynomial.coefficients.len(), (degree + 1) as usize);
    }
}