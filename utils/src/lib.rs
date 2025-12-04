pub mod arrays;
pub mod asciimap;
pub mod inputs;

pub fn swap<T: Copy>(vec: &mut Vec<T>, i: usize, j: usize) {
    let a = vec[i];
    vec[i] = vec[j];
    vec[j] = a;
}
