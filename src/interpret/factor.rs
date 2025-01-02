// All functions in this file were adapted from `num-integer` crate.
// `num-integer` is dual-licensed under MIT or Apache-2.0 like this crate.
// Downstream users will use `num-integer` derived code under the same license as the rest of this crate.
// MIT: https://github.com/rust-num/num-integer/blob/03640c2a9472fad6f40845ab29c7c9502935d1d3/LICENSE-MIT
// Apache-2.0: https://github.com/rust-num/num-integer/blob/03640c2a9472fad6f40845ab29c7c9502935d1d3/LICENSE-APACHE

// https://github.com/rust-num/num-integer/blob/03640c2a9472fad6f40845ab29c7c9502935d1d3/src/lib.rs#L868-L895
pub(super) fn gcd(mut first: usize, mut second: usize) -> usize {
    let ones = first | second;
    if first == 0 || second == 0 {
        return ones;
    }

    let shift = ones.trailing_zeros();
    first >>= first.trailing_zeros();
    second >>= second.trailing_zeros();

    while first != second {
        if first > second {
            first -= second;
            first >>= first.trailing_zeros();
        } else {
            second -= first;
            second >>= second.trailing_zeros();
        }
    }

    first << shift
}

// https://github.com/rust-num/num-integer/blob/03640c2a9472fad6f40845ab29c7c9502935d1d3/src/lib.rs#L909-L913
pub(super) fn lcm(first: usize, second: usize) -> usize {
    gcd_lcm(first, second).1
}

// https://github.com/rust-num/num-integer/blob/03640c2a9472fad6f40845ab29c7c9502935d1d3/src/lib.rs#L915-L925
fn gcd_lcm(first: usize, second: usize) -> (usize, usize) {
    if first == 0 && second == 0 {
        return (0, 0);
    }
    let gcd = gcd(first, second);
    let lcm = first * (second / gcd);
    (gcd, lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 3), 3);
        assert_eq!(gcd(3, 0), 3);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(10, 2), 2);
        assert_eq!(gcd(10, 3), 1);
        assert_eq!(gcd(56, 42), 14);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(0, 0), 0);
        assert_eq!(lcm(0, 3), 0);
        assert_eq!(lcm(3, 0), 0);
        assert_eq!(lcm(3, 3), 3);
        assert_eq!(lcm(11, 22), 22);
        assert_eq!(lcm(8, 9), 72);
        assert_eq!(lcm(11, 5), 55);
        assert_eq!(lcm(15, 17), 255);
    }
}
