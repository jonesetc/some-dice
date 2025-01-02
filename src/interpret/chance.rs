use std::ops;

use super::factor;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct Chance {
    numerator: usize,
    denominator: usize,
}

impl ops::Mul<Self> for Chance {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<Self> for Chance {
    fn mul_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
        self.simplify();
    }
}

impl ops::Add<Self> for Chance {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign<Self> for Chance {
    fn add_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.denominator;
        self.numerator += self.denominator * rhs.numerator;
        self.denominator *= rhs.denominator;
        self.simplify();
    }
}

impl ops::Mul<usize> for Chance {
    type Output = Self;

    fn mul(mut self, rhs: usize) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<usize> for Chance {
    fn mul_assign(&mut self, rhs: usize) {
        self.numerator *= rhs;
        self.simplify();
    }
}

impl ops::Div<usize> for Chance {
    type Output = Self;

    fn div(mut self, rhs: usize) -> Self::Output {
        self /= rhs;
        self
    }
}

impl ops::DivAssign<usize> for Chance {
    // Rational devision is implemented by multiplying the denominator
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: usize) {
        self.denominator *= rhs;
        self.simplify();
    }
}

impl Chance {
    pub(super) fn new(numerator: usize, denominator: usize) -> Chance {
        let mut result = Chance {
            numerator,
            denominator,
        };
        result.simplify();

        result
    }

    fn simplify(&mut self) {
        if self.denominator <= 1 {
            return;
        }

        let factor = factor::gcd(self.numerator, self.denominator);

        if factor > 1 {
            self.numerator /= factor;
            self.denominator /= factor;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_simplify() {
        assert_eq!(
            Chance::new(6, 9),
            Chance {
                numerator: 2,
                denominator: 3
            }
        );

        assert_eq!(
            Chance::new(3, 5),
            Chance {
                numerator: 3,
                denominator: 5
            }
        );

        assert_eq!(
            Chance::new(3, 3),
            Chance {
                numerator: 1,
                denominator: 1
            }
        );
    }

    #[test]
    fn test_multiply_by_chance() {
        assert_eq!(Chance::new(1, 2) * Chance::new(1, 1), Chance::new(1, 2));
        assert_eq!(Chance::new(1, 2) * Chance::new(1, 3), Chance::new(1, 6));
        assert_eq!(Chance::new(1, 3) * Chance::new(3, 4), Chance::new(1, 4));
    }

    #[test]
    fn test_add_to_chance() {
        assert_eq!(Chance::new(1, 2) + Chance::new(1, 2), Chance::new(1, 1));
        assert_eq!(Chance::new(1, 3) + Chance::new(1, 6), Chance::new(1, 2));
        assert_eq!(Chance::new(1, 3) + Chance::new(1, 5), Chance::new(8, 15));
    }

    #[test]
    fn test_multiply_by_scalar() {
        assert_eq!(Chance::new(1, 2) * 2, Chance::new(1, 1));
        assert_eq!(Chance::new(1, 3) * 2, Chance::new(2, 3));
        assert_eq!(Chance::new(1, 3) * 1, Chance::new(1, 3));
    }

    #[test]
    fn test_divide_by_scalar() {
        assert_eq!(Chance::new(1, 1) / 2, Chance::new(1, 2));
        assert_eq!(Chance::new(2, 3) / 2, Chance::new(1, 3));
        assert_eq!(Chance::new(1, 3) / 1, Chance::new(1, 3));
    }
}
