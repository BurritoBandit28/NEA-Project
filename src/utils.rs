use num::integer::sqrt;
use num::pow;

pub fn mul_vec(vec : &mut (f32, f32), val : f32) {
    vec.0 *= val;
    vec.1 *= val;
}

pub fn normalise_vec(vec : &mut (f32, f32)) {
    // get the square root of the object
    let mag = f32::sqrt(vec.0 * vec.0) + (vec.1 * vec.1);
    vec.0 /= mag;
    vec.1 /= mag;
}