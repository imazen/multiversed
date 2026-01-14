//! Test crate using multiversed with default features (x86-64-v3, aarch64-baseline)

use multiversed::multiversed;

/// Simple function that gets multiversioned with default preset targets
#[multiversed]
pub fn add_arrays(a: &[f32], b: &[f32], out: &mut [f32]) {
    for ((o, a), b) in out.iter_mut().zip(a.iter()).zip(b.iter()) {
        *o = *a + *b;
    }
}

/// Function with explicit return type
#[multiversed]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_arrays() {
        let a = [1.0f32, 2.0, 3.0, 4.0];
        let b = [5.0f32, 6.0, 7.0, 8.0];
        let mut out = [0.0f32; 4];
        add_arrays(&a, &b, &mut out);
        assert_eq!(out, [6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0f32, 2.0, 3.0, 4.0];
        let b = [1.0f32, 1.0, 1.0, 1.0];
        let result = dot_product(&a, &b);
        assert_eq!(result, 10.0);
    }
}
