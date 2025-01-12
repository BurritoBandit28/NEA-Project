// This file contains all the tests for this project.


#[cfg(test)]
mod tests {
    use num::integer::sqrt;
    use crate::utils::{mul_vec, normalise_vec};
    use super::*;

    #[test]
    fn mul_vec_test() {
        let mut vector = (3.5, 6.0);
        mul_vec(&mut vector, 4.0);
        assert_eq!(vector, (14.0, 24.0))
    }

    #[test]
    fn normalise_vec_test() {
        let vector = (3.0, 4.0);
        let mag = f32::sqrt((vector.0 * vector.0) + (vector.1 * vector.1));
        assert_eq!(mag, 5.0);
        let normalised = normalise_vec(vector);
        let mag_normalised = f32::sqrt((normalised.0 * normalised.0) + (normalised.1 * normalised.1));
        assert_eq!(mag_normalised, 1.0)
    }
}