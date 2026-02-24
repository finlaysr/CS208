use rand::prelude::*;

pub fn random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut ret = vec![];
    for _ in 0..size {
        ret.push(rng.random_range(-1000..=1000));
    }
    ret
}
